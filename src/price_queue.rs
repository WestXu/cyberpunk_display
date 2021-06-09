use std::collections::vec_deque::VecDeque;
use std::fmt;

use ordered_float::NotNan;

pub enum Direction {
    Flat,
    Up,
    Down,
}

#[derive(Debug)]
pub struct PriceQueue {
    q: VecDeque<NotNan<f64>>,
}

impl PriceQueue {
    pub fn new() -> PriceQueue {
        let q = VecDeque::with_capacity(32);
        PriceQueue { q }
    }

    pub fn push(&mut self, p: NotNan<f64>) {
        if self.q.len() == 32 {
            self.q.pop_front();
        }
        self.q.push_back(p);
        while self.q.len() < 32 {
            self.q.push_back(p);
        }
    }

    pub fn get_up_down(&self) -> Vec<Direction> {
        let mut v = vec![Direction::Flat];
        for i in 1..self.q.len() {
            let p = self.q.get(i);
            let pre_p = self.q.get(i - 1);

            let d: Direction = if p == pre_p {
                Direction::Flat
            } else if p > pre_p {
                Direction::Up
            } else {
                Direction::Down
            };

            v.push(d);
        }
        v
    }

    #[cfg(test)]
    pub fn get_up_down_repr(&self) -> String {
        self.get_up_down()
            .iter()
            .map(|d| match d {
                Direction::Flat => '-',
                Direction::Up => '↑',
                Direction::Down => '↓',
            })
            .collect()
    }

    pub fn get_int_pos_v(&self) -> Vec<u8> {
        let (min, max) = (self.q.iter().min().unwrap(), self.q.iter().max().unwrap());
        let rng: NotNan<f64> = max - min;
        if rng == 0.0 {
            vec![3; 32]
        } else {
            self.q
                .iter()
                .map(|p| ((p - min) / rng * 6.0).round() as u8)
                .collect()
        }
    }
}

impl fmt::Display for PriceQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.get_int_pos_v())
    }
}
