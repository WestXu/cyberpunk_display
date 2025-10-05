use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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
                32usize.saturating_sub(major_cs.pixels[0].len() + 1),
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
                32usize.saturating_sub(minor_cs.pixels[0].len() + 1),
                5,
            );
        }
        screen
    }
}

async fn wait_for_round_second() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let sub = now.subsec_nanos(); // 0..=999_999_999
    let ns = 1_000_000_000 - sub as u64;
    tokio::time::sleep(Duration::from_nanos(ns)).await;
}

pub struct BtcTimeMatrix {
    pq: PriceQueue,
    ws_coin: WsCoin,
    price: Option<Decimal>,
    last_blink: Instant,
    indicator_lit: bool, // a "network activity" indicator at bottom-left corner
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
            last_blink: Instant::now(),
            indicator_lit: false,
        }
    }
    pub async fn gen_screen(&mut self) -> Screen {
        tokio::select! {
            Some(price) = self.ws_coin.next() => {
                self.price = Some(price.price);
                self.pq.push(price.price);
                if self.last_blink.elapsed() > Duration::from_millis(100) { // don't blink too fast
                    self.indicator_lit = !self.indicator_lit; // toggle the indicator on new price
                    self.last_blink = Instant::now();
                }

            },
            _ = wait_for_round_second() => {
                self.indicator_lit = false; // turn off the indicator after timeout
            }
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
                32usize.saturating_sub(major_cs.pixels[0].len() + 1),
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
            32usize.saturating_sub(minor_cs.pixels[0].len() + 1),
            5,
        );

        if self.indicator_lit {
            screen.draw(&[vec![Some(Rgb888::new(255, 255, 0))]], 0, 7);
        }

        screen
    }
}
