use ordered_float::NotNan;
use serde::Deserialize;

#[derive(Debug)]
pub enum Msg {
    Ping(u64),
    Subscribed(String),
    Price(NotNan<f64>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Received {
    Ping {
        ping: u64,
    },
    Subscribed {
        // id: String,
        status: String,
        subbed: String,
        // ts: u64,
    },
    Price {
        // ch: String,
        // ts: u64,
        tick: Tick,
    },
}

#[derive(Deserialize, Debug)]
struct Tick {
    // id: u64,
    // ts: u64,
    data: Vec<TickData>,
}

#[derive(Deserialize, Debug)]
struct TickData {
    // id: u128, an open issue https://github.com/serde-rs/serde/issues/1682
    // ts: u64,
    // tradeId: u64,
    // amount: f64,
    price: f64,
    // direction: String,
}

pub fn parse_json(data: &str) -> Msg {
    let fail_msg = format!("Failed parsing data: {}", data);

    let v: Received = serde_json::from_str(data).expect(&fail_msg);

    match v {
        Received::Ping { ping } => Msg::Ping(ping),
        Received::Subscribed { status, subbed } => {
            assert_eq!(status, "ok", "{}", fail_msg);
            Msg::Subscribed(subbed)
        }
        Received::Price { tick } => Msg::Price(NotNan::new(tick.data[0].price).expect(&fail_msg)),
    }
}

#[test]
fn test_parse_json() {
    let msgs: Vec<Received> = serde_json::from_str(
        r#"
        [
            {
                "id": "btcusdt",
                "status": "ok",
                "subbed": "market.btcusdt.trade.detail",
                "ts": 1624332964918
            },
            {
                "ch": "market.btcusdt.trade.detail",
                "ts": 1624332964575,
                "tick": {
                    "id": 131421049089,
                    "ts": 1624332964573,
                    "data": [
                        {
                            "id": 131421049089304937968219549,
                            "ts": 1624332964573,
                            "tradeId": 102482210043,
                            "amount": 0.006077,
                            "price": 32942.44,
                            "direction": "sell"
                        }
                    ]
                }
            },
            {
                "ping": 1624332968042
            }
        ]
        "#,
    )
    .unwrap();
    println!("{:#?}", msgs);
}
