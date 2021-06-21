use ordered_float::NotNan;
use rand::Rng;
use tungstenite::{connect, Message};
use url::Url;

use flate2::read::GzDecoder;
use std::io::Read;

use cyberpunk_display::price_queue::PriceQueue;

fn main() {
    let mut rng = rand::thread_rng();

    let mut pq = PriceQueue::default();

    let mut p = NotNan::new(50000.0).unwrap();

    for _i in 1..20 {
        p += NotNan::new(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);
        println!("{:}", pq);
        println!("\n");
    }

    env_logger::init();
    let (mut socket, response) =
        connect(Url::parse("wss://api.hadax.com/ws").unwrap()).expect("Can't connect");
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
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
        println!("Received: {}", s);
    }
    // socket.close(None);
}
