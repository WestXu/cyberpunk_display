pub mod parse_json;

use serde_json::json;

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
