pub mod parse_json;

use futures::{SinkExt, Stream, StreamExt};
use ordered_float::NotNan;
use parse_json::{parse_json, Msg};
use std::{error::Error, fmt, time::Duration};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

type PriceSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

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
    pub socket: PriceSocket,
}

pub async fn connect(markets: &[Market]) -> anyhow::Result<PriceSocket> {
    println!("Connecting to Binance WebSocket...");
    let url = Url::parse("wss://data-stream.binance.com/ws/test")?;
    let (mut socket, _) = connect_async(url).await?;
    println!("Connected to Binance WebSocket");

    let msg = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": markets.iter().map(|m| format!("{}@aggTrade", m.symbol.to_lowercase())).collect::<Vec<String>>(),
        "id": 1
    })
    .to_string();
    socket.send(Message::Text(msg)).await?;
    Ok(socket)
}

impl WsCoin {
    pub async fn default() -> Self {
        let markets = vec![Market {
            symbol: "BTCUSDT".to_string(),
            name: "BTC".to_string(),
        }];
        WsCoin {
            socket: connect(&markets).await.unwrap(),
            markets,
        }
    }
    async fn connect(&mut self) {
        loop {
            match connect(&self.markets).await {
                Ok(socket) => {
                    self.socket = socket;
                    break;
                }
                Err(error) => {
                    println!("error happened during connection: {error}, retrying...");
                }
            }
        }
    }

    async fn reconnect(&mut self) {
        println!("Reconnect in 60s...");
        tokio::time::sleep(Duration::from_secs(60)).await;
        println!("Reconnecting...");
        self.connect().await;
        println!("Reconnected. \n\n\n\n\n\n\n\n");
    }

    async fn recv_price(&mut self) -> Result<Price, RecvError> {
        while let Some(msg) = self.socket.next().await {
            match msg {
                Ok(Message::Text(msg)) => match parse_json(&msg) {
                    Ok(msg) => match msg {
                        Msg::Subscribed => continue,
                        Msg::Price { symbol, price: p } => {
                            return Ok(Price {
                                name: {
                                    let mut name = None;
                                    for market in &self.markets {
                                        if market.symbol == symbol {
                                            name = Some(market.name.clone());
                                            break;
                                        }
                                    }
                                    name.unwrap_or_else(|| {
                                        panic!("market name not found for symbol {}", symbol)
                                    })
                                },
                                price: p,
                            })
                        }
                    },
                    Err(error) => panic!("Error {} happened parsing json: {}", error, &msg),
                },
                Ok(Message::Ping(_)) => {
                    self.socket
                        .send(Message::Pong(vec![]))
                        .await
                        .expect("failed sending pong");
                    continue;
                }
                Ok(msg) => {
                    return Err(RecvError::UnexpectedMsgError(format!(
                        "Unexpected message received: {msg}"
                    )))
                }
                Err(error) => return Err(RecvError::RecevingError(error.to_string())),
            }
        }
        Err(RecvError::RecevingError("Disconnected".to_string()))
    }

    fn subscribe(&mut self) -> impl Stream<Item = Price> + '_ {
        async_stream::stream! {
            loop {
                match self.recv_price().await {
                    Ok(price) => yield price,
                    Err(error) => {
                        println!("{}\n\n\n\n\n\n\n\n", error);
                        self.reconnect().await
                    }
                }
            }
        }
    }
}

impl Stream for WsCoin {
    type Item = Price;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let stream = self.get_mut().subscribe();
        tokio::pin!(stream);
        stream.poll_next(cx)
    }
}
