pub mod parse_json;

use native_tls::TlsConnector;
use std::error::Error;
use std::fmt;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use tungstenite::{client, Message};
use url::Url;

use ordered_float::NotNan;

use parse_json::{parse_json, Msg};

#[derive(Debug)]
pub struct Price {
    pub name: String,
    pub price: NotNan<f64>,
}

pub struct Market {
    pub symbol: String,
    pub name: String,
}

#[derive(Debug)]
pub enum RecvError {
    RecevingError(String),
    UnexpectedMsgError(String),
    ParsingError(String),
}

impl Error for RecvError {}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecvError::RecevingError(err_str) => write!(f, "{}", err_str),
            RecvError::UnexpectedMsgError(err_str) => write!(f, "{}", err_str),
            RecvError::ParsingError(err_str) => write!(f, "{}", err_str),
        }
    }
}

pub struct WsCoin {
    pub markets: Vec<Market>,
    pub socket: Option<tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>>,
}

impl Default for WsCoin {
    fn default() -> Self {
        WsCoin {
            markets: vec![Market {
                symbol: "BTCUSDT".to_string(),
                name: "BTC".to_string(),
            }],
            socket: None,
        }
    }
}

impl Iterator for WsCoin {
    type Item = Price;
    fn next(&mut self) -> Option<Self::Item> {
        match self.recv_price() {
            Ok(price) => Some(price),
            Err(error) => {
                println!("{}\n\n\n\n\n\n\n\n", error);
                match error {
                    RecvError::RecevingError(_) => {
                        self.reconnect();
                        self.next()
                    }
                    RecvError::UnexpectedMsgError(_) | RecvError::ParsingError(_) => self.next(),
                }
            }
        }
    }
}

fn connect(
    markets: &[Market],
) -> Result<tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>, Box<dyn Error>> {
    let stream = TcpStream::connect("data-stream.binance.com:443")?;
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    let stream = TlsConnector::new()?.connect("data-stream.binance.com", stream)?;
    let (mut socket, _) = client(Url::parse("wss://data-stream.binance.com/ws/test")?, stream)?;

    let msg = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": markets.iter().map(|m| format!("{}@aggTrade", m.symbol.to_lowercase())).collect::<Vec<String>>(),
        "id": 1
    })
    .to_string();
    socket.write_message(Message::Text(msg))?;

    Ok(socket)
}

impl WsCoin {
    fn connect(&mut self) {
        match connect(&self.markets) {
            Ok(socket) => self.socket = Some(socket),
            Err(error) => {
                println!("{}", error);
                self.reconnect();
            }
        };
    }

    fn reconnect(&mut self) {
        println!("Reconnect in 60s...");
        thread::sleep(Duration::from_secs(60));
        println!("Reconnecting...");
        self.connect();
        println!("Reconnected. \n\n\n\n\n\n\n\n");
    }

    fn recv_price(&mut self) -> Result<Price, RecvError> {
        let socket = match self.socket.as_mut() {
            None => {
                self.connect();
                self.socket.as_mut().unwrap()
            }
            Some(socket) => socket,
        };

        match socket.read_message() {
            Ok(Message::Text(msg)) => match parse_json(&msg) {
                Ok(msg) => match msg {
                    Msg::Subscribed => self.recv_price(),
                    Msg::Price { symbol, price: p } => Ok(Price {
                        name: {
                            let mut name = None;
                            for market in &self.markets {
                                if market.symbol == symbol {
                                    name = Some(market.name.clone());
                                    break;
                                }
                            }
                            name.expect(&format!("market name not found for symbol {}", symbol))
                        },
                        price: p,
                    }),
                },
                Err(error) => panic!("Error {} happened parsing json: {}", error, &msg),
            },
            Ok(Message::Ping(_)) => {
                self.socket
                    .as_mut()
                    .unwrap()
                    .write_message(Message::Pong(vec![]))
                    .expect("failed sending pong");
                self.recv_price()
            }
            Ok(_) => Err(RecvError::UnexpectedMsgError(
                "Unexpected message received".to_string(),
            )),
            Err(error) => {
                panic!("Error {} happened receiving", error);
            }
        }
    }
}
