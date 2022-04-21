use super::price_queue::{PlotKind, PriceQueue};
use super::screen::character::{Character, Font};
use super::screen::rgb::{colorize, Rgb888};
use super::screen::Screen;
use super::ws_coin::{Market, Price, WsCoin};
use ordered_float::NotNan;

#[derive(Default)]
pub struct BtcMatrix {
    pq: PriceQueue,
    ws_coin: WsCoin,
}

impl Iterator for BtcMatrix {
    type Item = Screen;
    fn next(&mut self) -> Option<Self::Item> {
        let p = self.ws_coin.next().unwrap().price;
        self.pq.push(p);
        Some(self.pq.to_screen(PlotKind::TrendLine, true))
    }
}

pub struct BtcEthMatrix {
    pq: PriceQueue,
    ws_coin: WsCoin,
    btc_price: Option<NotNan<f64>>,
    eth_price: Option<NotNan<f64>>,
}

impl Default for BtcEthMatrix {
    fn default() -> Self {
        BtcEthMatrix {
            pq: PriceQueue::default(),
            ws_coin: WsCoin {
                markets: vec![
                    Market {
                        symbol: "BTC/USD".to_string(),
                        name: "BTC".to_string(),
                    },
                    Market {
                        symbol: "ETH/USD".to_string(),
                        name: "ETH".to_string(),
                    },
                ],
                socket: None,
                last_ping_time: None,
            },
            btc_price: None,
            eth_price: None,
        }
    }
}

impl Iterator for BtcEthMatrix {
    type Item = Screen;
    fn next(&mut self) -> Option<Self::Item> {
        let price: Price = self.ws_coin.next().unwrap();
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
                    &Rgb888::new(170, 170, 170),
                ),
                32 - (minor_cs.pixels[0].len() + 1),
                5,
            );
        }
        Some(screen)
    }
}
