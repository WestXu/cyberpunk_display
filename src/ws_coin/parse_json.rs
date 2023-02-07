use std::error::Error;

use ordered_float::NotNan;
use serde::Deserialize;

#[derive(Debug)]
pub enum Msg {
    Subscribed,
    Price { symbol: String, price: NotNan<f64> },
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged, rename_all = "snake_case")]
enum Received {
    Update {
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "p")]
        price: String,
    },
    Subscribed {
        id: u8,
    },
}

pub fn parse_json(data: &str) -> Result<Msg, Box<dyn Error>> {
    Ok(match serde_json::from_str::<Received>(data)? {
        Received::Subscribed { .. } => Msg::Subscribed,
        Received::Update { symbol, price } => Msg::Price {
            symbol,
            price: NotNan::new(price.parse().unwrap())?,
        },
    })
}

#[test]
fn test_parse_json() {
    let msgs: Vec<Received> = serde_json::from_str(
        r#"
        [
            {
                "result": null,
                "id": 1
            },
            {
                "e": "aggTrade",
                "E": 123456789,
                "s": "BNBBTC",
                "a": 12345,
                "p": "0.001",
                "q": "100",
                "f": 100,
                "l": 105,
                "T": 123456785,
                "m": true,
                "M": true
            }
        ]
        "#,
    )
    .unwrap();

    assert_eq!(
        msgs,
        [
            Received::Subscribed { id: 1 },
            Received::Update {
                symbol: "BNBBTC".to_string(),
                price: "0.001".to_string(),
            },
        ]
    );
}
