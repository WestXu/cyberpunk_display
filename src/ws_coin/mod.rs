pub mod parse_json;

use std::thread;
use std::time::Duration;

use tungstenite::{connect, Message};
use url::Url;

use ordered_float::NotNan;
use serde_json::json;

use flate2::read::GzDecoder;
use std::io::Read;

use parse_json::{parse_json, Msg};

pub struct Price {
    pub name: String,
    pub price: NotNan<f64>,
}

pub struct Market {
    symbol: String,
    name: String,
}

pub struct WsCoin {
    markets: Vec<Market>,
    socket: Option<
        tungstenite::WebSocket<
            tungstenite::stream::Stream<
                std::net::TcpStream,
                native_tls::TlsStream<std::net::TcpStream>,
            >,
        >,
    >,
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
                println!("Error {} happened receiving", error);
                self.reconnect();
                return self.next();
            }
        };
        let msg_binary = msg.into_data();
        let mut gz = GzDecoder::new(&msg_binary[..]);
        let mut s = String::new();

        if let Err(error) = gz.read_to_string(&mut s) {
            println!("Error {} happened decompressing", error);
            return self.next();
        };
        match parse_json(&s) {
            Ok(msg) => match msg {
                Msg::Ping(ping) => {
                    self.pong(ping);
                    self.next()
                }
                Msg::Subscribed(_) => self.next(),
                Msg::Price { symbol, price: p } => Some(Price {
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
            Err(error) => {
                println!("Error {} happened parsing json: {}", error, &s);
                self.next()
            }
        }
    }
}

impl WsCoin {
    fn connect(
        &self,
    ) -> tungstenite::WebSocket<
        tungstenite::stream::Stream<
            std::net::TcpStream,
            native_tls::TlsStream<std::net::TcpStream>,
        >,
    > {
        let (mut socket, _) =
            connect(Url::parse("wss://api.hadax.com/ws").unwrap()).expect("Can't connect");

        for market in &self.markets {
            socket
                .write_message(Message::Text(format!(
                    "{{\"sub\": \"market.{}.trade.detail\", \"id\": \"{}\"}}",
                    market.symbol, market.symbol
                )))
                .unwrap();
        }

        socket
    }

    fn reconnect(&mut self) {
        thread::sleep(Duration::from_secs(60));
        self.socket = Some(self.connect())
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
