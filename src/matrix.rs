use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::{
    price_queue::{PlotKind, PriceQueue},
    screen::{
        character::{Character, Font},
        rgb::{colorize, Rgb888},
        Screen,
    },
    ws_coin::{Market, Price, WsCoin},
};
use futures::{Stream, StreamExt as _};
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
        let ws_coin = self.ws_coin.subscribe();
        tokio::pin!(ws_coin);
        let price: Price = ws_coin.next().await.unwrap();
        self.pq.push(price.price);
        self.pq.to_screen(PlotKind::FlatLine, false)
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
            indicator_lit: false,
        }
    }
    pub async fn gen_screen(&mut self) -> Screen {
        let ws_coin = self.ws_coin.subscribe();
        tokio::pin!(ws_coin);
        tokio::select! {
            Some(price) = ws_coin.next() => {
                self.price = Some(price.price);
                self.pq.push(price.price);

                self.indicator_lit = !self.indicator_lit; // toggle the indicator on new price
            },
            _ = wait_for_round_second() => {
                self.indicator_lit = false; // turn off the indicator at each second
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
    pub fn subscribe(&mut self) -> impl Stream<Item = Screen> + '_ {
        async_stream::stream! {
            loop {
                let screen = self.gen_screen().await;
                yield screen;
            }
        }
    }
}
