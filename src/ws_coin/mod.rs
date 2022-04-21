pub mod parse_json;

use native_tls::TlsConnector;
use std::error::Error;
use std::fmt;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

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
    DecodingError(String),
    ParsingError(String),
}

impl Error for RecvError {}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecvError::RecevingError(err_str) => write!(f, "{}", err_str),
            RecvError::DecodingError(err_str) => write!(f, "{}", err_str),
            RecvError::ParsingError(err_str) => write!(f, "{}", err_str),
        }
    }
}

pub struct WsCoin {
    pub markets: Vec<Market>,
    pub socket: Option<tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>>,
    pub last_ping_time: Option<SystemTime>,
}

impl Default for WsCoin {
    fn default() -> Self {
        WsCoin {
            markets: vec![Market {
                symbol: "BTC/USD".to_string(),
                name: "BTC".to_string(),
            }],
            socket: None,
            last_ping_time: None,
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
                    RecvError::DecodingError(_) | RecvError::ParsingError(_) => self.next(),
                }
            }
        }
    }
}

fn connect(
    markets: &[Market],
) -> Result<tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>, Box<dyn Error>> {
    let stream = TcpStream::connect("ftx.cool:443")?;
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    let stream = TlsConnector::new()?.connect("ftx.cool", stream)?;
    let (mut socket, _) = client(Url::parse("wss://ftx.cool/ws")?, stream)?;
    for market in markets {
        socket.write_message(Message::Text(format!(
            "{{\"channel\": \"trades\", \"market\": \"{}\", \"op\": \"subscribe\"}}",
            market.symbol,
        )))?;
    }
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

        match self.last_ping_time {
            // ping if needed
            None => {
                self.last_ping_time = Some(SystemTime::now());
            }
            Some(_) => {
                if self.last_ping_time.unwrap().elapsed().unwrap().as_secs() > 15 {
                    socket
                        .write_message(Message::Text("{\"op\": \"ping\"}".to_owned()))
                        .unwrap();
                    self.last_ping_time = Some(SystemTime::now());
                    // println!("Ping");
                }
            }
        }

        let msg = match socket.read_message() {
            Ok(msg) => msg,
            Err(error) => {
                return Err(RecvError::RecevingError(format!(
                    "Error {} happened receiving",
                    error
                )));
            }
        };
        let msg_binary = msg.into_data();

        let s = match String::from_utf8(msg_binary) {
            Err(error) => {
                return Err(RecvError::DecodingError(format!(
                    "Error {} happened decoding",
                    error
                )))
            }
            Ok(s) => s,
        };

        match parse_json(&s) {
            Ok(msg) => match msg {
                Msg::Subscribed(_) => self.recv_price(),
                Msg::Price { symbol, price: p } => Ok(Price {
                    name: {
                        let mut name = None;
                        for market in &self.markets {
                            if market.symbol == symbol {
                                name = Some(market.name.clone());
                                break;
                            }
                        }
                        name.unwrap()
                    },
                    price: p,
                }),
                Msg::Pong {} => {
                    // println!("Pong");
                    self.recv_price()
                }
            },
            Err(error) => Err(RecvError::ParsingError(format!(
                "Error {} happened parsing json: {}",
                error, &s
            ))),
        }
    }
}
