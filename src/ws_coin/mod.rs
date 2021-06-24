pub mod parse_json;

use tungstenite::{connect, Message};
use url::Url;

use ordered_float::NotNan;
use serde_json::json;

use flate2::read::GzDecoder;
use std::io::Read;

use parse_json::{parse_json, Msg};

pub struct WsCoin {
    socket: tungstenite::WebSocket<
        tungstenite::stream::Stream<
            std::net::TcpStream,
            native_tls::TlsStream<std::net::TcpStream>,
        >,
    >,
}

impl Default for WsCoin {
    fn default() -> Self {
        WsCoin {
            socket: Self::connect(),
        }
    }
}

impl Iterator for WsCoin {
    type Item = NotNan<f64>;
    fn next(&mut self) -> Option<Self::Item> {
        let msg = match self.socket.read_message() {
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
        gz.read_to_string(&mut s).unwrap();
        match parse_json(&s) {
            Ok(msg) => match msg {
                Msg::Ping(ping) => {
                    self.pong(ping);
                    self.next()
                }
                Msg::Subscribed(_) => self.next(),
                Msg::Price(p) => Some(p),
            },
            Err(error) => {
                println!("Error {} happened parsing json: {}", error, &s);
                self.next()
            }
        }
    }
}

impl WsCoin {
    fn connect() -> tungstenite::WebSocket<
        tungstenite::stream::Stream<
            std::net::TcpStream,
            native_tls::TlsStream<std::net::TcpStream>,
        >,
    > {
        let (mut socket, _) =
            connect(Url::parse("wss://api.hadax.com/ws").unwrap()).expect("Can't connect");

        socket
            .write_message(Message::Text(
                r#"{"sub": "market.btcusdt.trade.detail", "id": "btcusdt"}"#.into(),
            ))
            .unwrap();

        socket
    }

    fn reconnect(&mut self) {
        self.socket = Self::connect()
    }

    fn pong(&mut self, ping: u64) {
        if let Err(error) = self.socket.write_message(tungstenite::Message::Text(
            json!({ "pong": ping }).to_string(),
        )) {
            println!("Error {} happened ponging", error);
            self.reconnect()
        }
    }
}
