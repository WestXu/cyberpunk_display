use std::cmp::Ordering;
use std::collections::vec_deque::VecDeque;
use std::fmt;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use super::screen::{Rgb888, Screen};

#[derive(Copy, Clone)]
pub enum Direction {
    Flat,
    Up,
    Down,
}

pub enum PlotKind {
    TrendLine,
    FlatLine,
}

#[derive(Debug)]
pub struct PriceQueue {
    q: VecDeque<Decimal>,
}

impl Default for PriceQueue {
    fn default() -> Self {
        PriceQueue {
            q: VecDeque::with_capacity(32),
        }
    }
}

impl PriceQueue {
    pub fn push(&mut self, p: Decimal) {
        if self.q.len() == 32 {
            self.q.pop_front();
        }
        self.q.push_back(p);
        while self.q.len() < 32 {
            self.q.push_back(p);
        }
    }

    pub fn to_up_down(&self) -> Vec<Direction> {
        let mut v = vec![Direction::Flat];
        for i in 1..self.q.len() {
            let p = self.q.get(i);
            let pre_p = self.q.get(i - 1);

            let d: Direction = match p.cmp(&pre_p) {
                Ordering::Equal => Direction::Flat,
                Ordering::Greater => Direction::Up,
                Ordering::Less => Direction::Down,
            };

            v.push(d);
        }
        v
    }

    #[cfg(test)]
    pub fn to_up_down_repr(&self) -> String {
        self.to_up_down()
            .iter()
            .map(|d| match d {
                Direction::Flat => '-',
                Direction::Up => '↑',
                Direction::Down => '↓',
            })
            .collect()
    }

    pub fn to_int_pos_v(&self) -> Vec<usize> {
        let (min, max) = (self.q.iter().min().unwrap(), self.q.iter().max().unwrap());
        let rng: Decimal = max - min;
        if rng.is_zero() {
            vec![3; 32]
        } else {
            self.q
                .iter()
                .map(|p| ((p - min) / rng * dec!(7.0)).round().to_f64().unwrap() as usize)
                .collect()
        }
    }

    pub fn to_2d_array(&self) -> Vec<Vec<bool>> {
        let mut array = vec![vec![false; 32]; 8];

        for (col, i) in self.to_int_pos_v().iter().enumerate() {
            array[7 - i][col] = true;
        }

        array
    }

    fn to_2d_direction_array(&self) -> Vec<Vec<Option<Direction>>> {
        let up_down = self.to_up_down();

        self.to_2d_array()
            .iter()
            .map(|row| {
                row.iter()
                    .zip(&up_down)
                    .map(|(i, d)| match (i, d) {
                        (false, _) => None,
                        (true, d) => Some(*d),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn to_screen(&self, plot_kind: PlotKind, show_num: bool) -> Screen {
        let dim = 0.8;
        let dim_max = (255.0 * dim) as u8;
        let screen = match plot_kind {
            PlotKind::TrendLine => Screen {
                pixels: self
                    .to_2d_direction_array()
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|x| match x {
                                Some(Direction::Flat) => Some(Rgb888::new(0, 0, dim_max)),
                                Some(Direction::Up) => Some(Rgb888::new(0, dim_max, 0)),
                                Some(Direction::Down) => Some(Rgb888::new(dim_max, 0, 0)),
                                _ => None,
                            })
                            .collect()
                    })
                    .collect::<Vec<Vec<Option<Rgb888>>>>(),
            },
            PlotKind::FlatLine => {
                let mut screen = Screen::default();
                screen.draw(
                    &[self
                        .to_up_down()
                        .iter()
                        .map(|d| match d {
                            Direction::Flat => Some(Rgb888::new(0, 0, dim_max)),
                            Direction::Up => Some(Rgb888::new(0, dim_max, 0)),
                            Direction::Down => Some(Rgb888::new(dim_max, 0, 0)),
                        })
                        .collect()],
                    0,
                    4,
                );
                screen
            }
        };
        if show_num {
            screen + Screen::from_float(self.q[31])
        } else {
            screen
        }
    }
}

impl fmt::Display for PriceQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", self.to_screen(PlotKind::TrendLine, true))
    }
}
