pub mod parse_json;

use futures::{SinkExt, Stream, StreamExt};
use parse_json::{parse_json, Msg};
use rust_decimal::prelude::*;
use std::{error::Error, fmt, time::Duration};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

type PriceSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub struct Price {
    pub name: String,
    pub price: Decimal,
}

#[derive(Clone)]
pub struct Market {
    pub symbol: String,
    pub name: String,
}

#[derive(Debug)]
pub enum RecvError {
    Timeout,
    RecevingError(String),
    UnexpectedMsgError(String),
    ParsingError(String),
    Disconnected,
}

impl Error for RecvError {}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecvError::Timeout => write!(f, "Timeout"),
            RecvError::RecevingError(err_str) => write!(f, "{}", err_str),
            RecvError::UnexpectedMsgError(err_str) => write!(f, "{}", err_str),
            RecvError::ParsingError(err_str) => write!(f, "{}", err_str),
            RecvError::Disconnected => write!(f, "Disconnected"),
        }
    }
}

pub struct WsCoin {
    rx: tokio::sync::mpsc::UnboundedReceiver<Price>,
}
impl WsCoin {
    pub async fn new(markets: Vec<Market>) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            loop {
                let mut socket = match connect(&markets).await {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("Connection failed: {e}, retrying in 60s...");
                        tokio::time::sleep(Duration::from_secs(60)).await;
                        continue;
                    }
                };

                loop {
                    match recv_price(&mut socket, &markets).await {
                        Ok(price) => {
                            if tx.send(price).is_err() {
                                log::info!("Receiver dropped, exiting background task");
                                return;
                            }
                        }
                        Err(error) => {
                            log::error!(
                                "Error happened: {error}, Reconnect in 60s...\n\n\n\n\n\n\n\n",
                            );
                            tokio::time::sleep(Duration::from_secs(60)).await;
                            break;
                        }
                    }
                }
            }
        });

        WsCoin { rx }
    }
}

impl WsCoin {
    pub async fn default() -> Self {
        let markets = vec![Market {
            symbol: "BTCUSDT".to_string(),
            name: "BTC".to_string(),
        }];
        Self::new(markets).await
    }

    pub fn subscribe(&mut self) -> impl Stream<Item = Price> + '_ {
        async_stream::stream! {
            while let Some(price) = self.rx.recv().await {
                yield price;
            }
        }
    }
}

async fn connect(markets: &[Market]) -> anyhow::Result<PriceSocket> {
    log::info!("Connecting to Binance WebSocket...");
    let url = Url::parse("wss://stream.binance.com/ws")?;
    let (mut socket, _) = connect_async(url).await?;
    log::info!("Connected to Binance WebSocket");

    let msg = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": markets.iter().map(|m| format!("{}@aggTrade", m.symbol.to_lowercase())).collect::<Vec<String>>(),
        "id": 1
    })
    .to_string();
    log::debug!("Sending subscription message: {}", msg);
    socket.send(Message::Text(msg)).await?;
    log::debug!("Subscription message sent successfully");
    Ok(socket)
}

async fn recv_price(socket: &mut PriceSocket, markets: &[Market]) -> Result<Price, RecvError> {
    loop {
        let Some(msg) = tokio::time::timeout(Duration::from_secs(60), socket.next())
            .await
            .map_err(|_| RecvError::Timeout)?
        else {
            return Err(RecvError::Disconnected);
        };

        match msg {
            Ok(Message::Text(msg)) => match parse_json(&msg) {
                Ok(msg) => match msg {
                    Msg::Subscribed => log::info!("Subscribed confirmed"),
                    Msg::Price { symbol, price: p } => {
                        return Ok(Price {
                            name: {
                                let mut name = None;
                                for market in markets {
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
                Err(error) => {
                    panic!("Error {} happened parsing json: {}", error, &msg)
                }
            },
            Ok(Message::Ping(data)) => {
                if let Err(e) = socket.send(Message::Pong(data)).await {
                    log::error!("Failed to send pong: {}", e);
                    return Err(RecvError::Disconnected);
                }
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
}
