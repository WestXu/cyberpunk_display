use super::{
    price_queue::{PlotKind, PriceQueue},
    screen::{
        character::{Character, Font},
        rgb::{colorize, Rgb888},
        Screen,
    },
    ws_coin::{Market, Price, WsCoin},
};
use futures::StreamExt as _;
use rust_decimal::prelude::*;

pub struct BtcMatrix {
    pq: PriceQueue,
    ws_coin: WsCoin,
}

impl BtcMatrix {
    pub async fn default() -> Self {
        BtcMatrix {
            pq: PriceQueue::default(),
            ws_coin: WsCoin::default().await,
        }
    }
    pub async fn gen_screen(&mut self) -> Screen {
        let price: Price = self.ws_coin.next().await.unwrap();
        self.pq.push(price.price);
        self.pq.to_screen(PlotKind::FlatLine, false)
    }
}

pub struct BtcEthMatrix {
    pq: PriceQueue,
    ws_coin: WsCoin,
    btc_price: Option<Decimal>,
    eth_price: Option<Decimal>,
}

impl BtcEthMatrix {
    pub async fn default() -> Self {
        let markets = vec![
            Market {
                symbol: "BTCUSDT".to_string(),
                name: "BTC".to_string(),
            },
            Market {
                symbol: "ETHUSDT".to_string(),
                name: "ETH".to_string(),
            },
        ];
        BtcEthMatrix {
            pq: PriceQueue::default(),
            ws_coin: WsCoin::new(markets).await,
            btc_price: None,
            eth_price: None,
        }
    }

    pub async fn gen_screen(&mut self) -> Screen {
        let price: Price = self.ws_coin.next().await.unwrap();
        if price.name == "BTC" {
            self.btc_price = Some(price.price);
            self.pq.push(price.price);
        } else {
            self.eth_price = Some(price.price);
        }

        let mut screen = self.pq.to_screen(PlotKind::FlatLine, false);
        if self.btc_price.is_some() {
            let major_cs = Character::from_float(self.btc_price.unwrap(), Font::Medium);
            screen.draw(
                &colorize(
                    &major_cs.pixels,
                    &Rgb888::new(255, 255, 255),
                    &Rgb888::new(255, 255, 0),
                ),
                32 - (major_cs.pixels[0].len() + 1),
                0,
            );
        }
        if self.eth_price.is_some() {
            let minor_cs = Character::from_float(self.eth_price.unwrap(), Font::Small);
            screen.draw(
                &colorize(
                    &minor_cs.pixels,
                    &Rgb888::new(255, 255, 255),
                    &Rgb888::new(200, 200, 200),
                ),
                32 - (minor_cs.pixels[0].len() + 1),
                5,
            );
        }
        screen
    }
}

pub struct BtcTimeMatrix {
    pq: PriceQueue,
    ws_coin: WsCoin,
    price: Option<Decimal>,
}

impl BtcTimeMatrix {
    pub async fn default() -> Self {
        let markets = vec![Market {
            symbol: "BTCUSDT".to_string(),
            name: "BTC".to_string(),
        }];
        BtcTimeMatrix {
            pq: PriceQueue::default(),
            ws_coin: WsCoin::new(markets).await,
            price: None,
        }
    }
    pub async fn gen_screen(&mut self) -> Screen {
        tokio::select! {
            Some(price) = self.ws_coin.next() => {
                self.price = Some(price.price);
                self.pq.push(price.price);
            },
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {}
        }

        let mut screen = self.pq.to_screen(PlotKind::FlatLine, false);
        if self.price.is_some() {
            let major_cs = Character::from_float(self.price.unwrap(), Font::Medium);
            screen.draw(
                &colorize(
                    &major_cs.pixels,
                    &Rgb888::new(255, 255, 255),
                    &Rgb888::new(255, 255, 0),
                ),
                32 - (major_cs.pixels[0].len() + 1),
                0,
            );
        }

        let minor_cs = Character::from_time(Font::Small);
        screen.draw(
            &colorize(
                &minor_cs.pixels,
                &Rgb888::new(255, 255, 255),
                &Rgb888::new(200, 200, 200),
            ),
            32 - (minor_cs.pixels[0].len() + 1),
            5,
        );

        screen
    }
}
