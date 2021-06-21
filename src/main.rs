use tungstenite::{connect, Message};
use url::Url;

use flate2::read::GzDecoder;
use std::io::Read;

use cyberpunk_display::price_queue::PriceQueue;
use cyberpunk_display::ws_coin::{
    parse_json::{parse_json, Msg},
    pong,
};

fn main() {
    let mut pq = PriceQueue::default();

    let (mut socket, response) =
        connect(Url::parse("wss://api.hadax.com/ws").unwrap()).expect("Can't connect");

    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket
        .write_message(Message::Text(
            r#"{"sub": "market.btcusdt.trade.detail", "id": "btcusdt"}"#.into(),
        ))
        .unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg_binary = msg.into_data();
        let mut gz = GzDecoder::new(&msg_binary[..]);
        let mut s = String::new();
        gz.read_to_string(&mut s).unwrap();
        match parse_json(&s) {
            Msg::Ping(ping) => {
                pong(&mut socket, ping);
                continue;
            }
            Msg::Subscribed(ch) => println!("Parsed: {:?}", ch),
            Msg::Price(p) => {
                pq.push(p);
                println!("{}\n", pq)
            }
        }
    }
    // socket.close(None);
}
