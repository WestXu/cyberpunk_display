use std::error::Error;

use ordered_float::NotNan;
use serde::Deserialize;

#[derive(Debug)]
pub enum Msg {
    Subscribed(String),
    Price { symbol: String, price: NotNan<f64> },
    Pong {},
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
enum Received {
    subscribed {
        // channel: String,
        market: String,
    },
    update {
        // channel: String,
        market: String,
        data: Vec<TickData>,
    },
    pong {},
}

#[derive(Deserialize, PartialEq, Debug)]
struct TickData {
    // id: u128, an open issue https://github.com/serde-rs/serde/issues/1682
    // time: String,
    // liquidation: bool,
    // side: String,
    // size: f64,
    price: f64,
}

pub fn parse_json(data: &str) -> Result<Msg, Box<dyn Error>> {
    Ok(match serde_json::from_str::<Received>(data)? {
        Received::subscribed { market } => Msg::Subscribed(market),
        Received::update { market, data } => Msg::Price {
            symbol: market,
            price: NotNan::new(data[0].price)?,
        },
        Received::pong {} => Msg::Pong {},
    })
}

#[test]
fn test_parse_json() {
    let msgs: Vec<Received> = serde_json::from_str(
        r#"
        [
            {
                "type": "subscribed",
                "channel": "trades",
                "market": "BTC/USD"
            },
            {
                "channel": "trades",
                "market": "BTC/USD",
                "type": "update",
                "data": [
                    {
                        "id": 3789370083,
                        "price": 41624.0,
                        "size": 0.0001,
                        "side": "sell",
                        "liquidation": false,
                        "time": "2022-04-21T07:26:35.043893+00:00"
                    },
                    {
                        "id": 3789370084,
                        "price": 41624.0,
                        "size": 0.0007,
                        "side": "sell",
                        "liquidation": false,
                        "time": "2022-04-21T07:26:35.043893+00:00"
                    }
                ]
            },
            {"type": "pong"}
        ]
        "#,
    )
    .unwrap();

    assert_eq!(
        msgs,
        [
            Received::subscribed {
                market: "BTC/USD".to_string(),
            },
            Received::update {
                market: "BTC/USD".to_string(),
                data: vec!(TickData { price: 41624.0 }, TickData { price: 41624.0 }),
            },
            Received::pong {}
        ]
    );
}
