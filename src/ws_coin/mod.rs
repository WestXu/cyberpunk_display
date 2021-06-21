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
        let (mut socket, _) =
            connect(Url::parse("wss://api.hadax.com/ws").unwrap()).expect("Can't connect");

        socket
            .write_message(Message::Text(
                r#"{"sub": "market.btcusdt.trade.detail", "id": "btcusdt"}"#.into(),
            ))
            .unwrap();

        WsCoin { socket }
    }
}

impl Iterator for WsCoin {
    type Item = NotNan<f64>;
    fn next(&mut self) -> Option<Self::Item> {
        let msg = self.socket.read_message().expect("Error reading message");
        let msg_binary = msg.into_data();
        let mut gz = GzDecoder::new(&msg_binary[..]);
        let mut s = String::new();
        gz.read_to_string(&mut s).unwrap();
        match parse_json(&s) {
            Msg::Ping(ping) => {
                pong(&mut self.socket, ping);
                self.next()
            }
            Msg::Subscribed(ch) => {
                println!("Parsed: {:?}", ch);
                self.next()
            }
            Msg::Price(p) => {
                println!("Parsed: {:?}", p);
                Some(p)
            }
        }
    }
}

pub fn pong(
    socket: &mut tungstenite::WebSocket<
        tungstenite::stream::Stream<
            std::net::TcpStream,
            native_tls::TlsStream<std::net::TcpStream>,
        >,
    >,
    ping: u64,
) {
    socket
        .write_message(tungstenite::Message::Text(
            json!({ "pong": ping }).to_string(),
        ))
        .unwrap();
}
