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
use serde_json::json;

use flate2::read::GzDecoder;
use std::io::Read;

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
    DecopmressionError(String),
    ParsingError(String),
}

impl Error for RecvError {}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecvError::RecevingError(err_str) => write!(f, "{}", err_str),
            RecvError::DecopmressionError(err_str) => write!(f, "{}", err_str),
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
                symbol: "btcusdt".to_string(),
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
                    RecvError::DecopmressionError(_) | RecvError::ParsingError(_) => self.next(),
                }
            }
        }
    }
}

fn connect(
    markets: &[Market],
) -> Result<tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>, Box<dyn Error>> {
    let stream = TcpStream::connect("api.hadax.com:443")?;
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    let stream = TlsConnector::new()?.connect("api.hadax.com", stream)?;
    let (mut socket, _) = client(Url::parse("wss://api.hadax.com/ws")?, stream)?;
    for market in markets {
        socket.write_message(Message::Text(format!(
            "{{\"sub\": \"market.{}.trade.detail\", \"id\": \"{}\"}}",
            market.symbol, market.symbol
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
        let mut gz = GzDecoder::new(&msg_binary[..]);
        let mut s = String::new();

        if let Err(error) = gz.read_to_string(&mut s) {
            return Err(RecvError::DecopmressionError(format!(
                "Error {} happened decompressing",
                error
            )));
        };

        match parse_json(&s) {
            Ok(msg) => match msg {
                Msg::Ping(ping) => {
                    self.pong(ping);
                    self.recv_price()
                }
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
            },
            Err(error) => Err(RecvError::ParsingError(format!(
                "Error {} happened parsing json: {}",
                error, &s
            ))),
        }
    }

    fn pong(&mut self, ping: u64) {
        if let Err(error) = self
            .socket
            .as_mut()
            .unwrap()
            .write_message(tungstenite::Message::Text(
                json!({ "pong": ping }).to_string(),
            ))
        {
            println!("Error {} happened ponging", error);
            self.reconnect()
        }
    }
}
