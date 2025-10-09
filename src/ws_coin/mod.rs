pub mod parse_json;

use futures::{SinkExt, Stream, StreamExt};
use parse_json::{parse_json, Msg};
use rust_decimal::prelude::*;
use std::{error::Error, fmt, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Mutex};
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

enum ConnectionState {
    Connected(Box<PriceSocket>),
    Reconnecting,
}

#[derive(Clone)]
pub struct WsCoin {
    markets: Vec<Market>,
    connection: Arc<Mutex<ConnectionState>>,
}
impl WsCoin {
    pub async fn new(markets: Vec<Market>) -> Self {
        WsCoin {
            connection: Arc::new(Mutex::new(ConnectionState::Connected(Box::new(
                connect(&markets).await.unwrap(),
            )))),
            markets,
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

impl WsCoin {
    pub async fn default() -> Self {
        let markets = vec![Market {
            symbol: "BTCUSDT".to_string(),
            name: "BTC".to_string(),
        }];
        Self::new(markets).await
    }

    async fn reconnect(&self) {
        {
            let mut connection = self.connection.lock().await;

            if matches!(*connection, ConnectionState::Reconnecting) {
                return;
            }

            *connection = ConnectionState::Reconnecting;
        }

        log::info!("Reconnect in 60s...");
        tokio::time::sleep(Duration::from_secs(60)).await;
        log::info!("Reconnecting...");

        loop {
            match connect(&self.markets).await {
                Ok(new_socket) => {
                    log::debug!("reconnect: Successfully connected, updating connection state");
                    *self.connection.lock().await =
                        ConnectionState::Connected(Box::new(new_socket));
                    break;
                }
                Err(error) => {
                    log::info!("error happened during connection: {error}, retrying...");
                }
            }
        }
        log::info!("Reconnected. \n\n\n\n\n\n\n\n");
    }

    async fn recv_price(&mut self) -> Result<Price, RecvError> {
        loop {
            let mut connection = self.connection.lock().await;
            match &mut *connection {
                ConnectionState::Reconnecting => {
                    log::debug!("recv_price: Connection is reconnecting, waiting...");
                    drop(connection);
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    continue;
                }
                ConnectionState::Connected(socket) => {
                    if let Some(msg) = tokio::time::timeout(Duration::from_secs(60), socket.next())
                        .await
                        .map_err(|_| RecvError::Timeout)?
                    {
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
                                                    panic!(
                                                        "market name not found for symbol {}",
                                                        symbol
                                                    )
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
                    } else {
                        return Err(RecvError::Disconnected);
                    }
                }
            }
        }
    }

    fn subscribe(&mut self) -> impl Stream<Item = Price> + '_ {
        async_stream::stream! {
            loop {
                match self.recv_price().await {
                    Ok(price) => yield price,
                    Err(error) => {
                        match error {
                            RecvError::Disconnected => (),
                            _ => {
                                log::error!("Error happened: {}\n\n\n\n\n\n\n\n", error);
                            }
                        }
                        let _self = self.clone();
                        tokio::spawn(async move { _self.reconnect().await });
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
