use std::cmp::Ordering;
use std::collections::vec_deque::VecDeque;
use std::fmt;

use ordered_float::NotNan;

use super::screen::{Rgb888, Screen};

#[derive(Copy, Clone)]
pub enum Direction {
    Flat,
    Up,
    Down,
}

#[derive(Debug)]
pub struct PriceQueue {
    q: VecDeque<NotNan<f64>>,
}

impl Default for PriceQueue {
    fn default() -> Self {
        PriceQueue {
            q: VecDeque::with_capacity(32),
        }
    }
}

impl PriceQueue {
    pub fn push(&mut self, p: NotNan<f64>) {
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
        let rng: NotNan<f64> = max - min;
        if rng == 0.0 {
            vec![3; 32]
        } else {
            self.q
                .iter()
                .map(|p| ((p - min) / rng * 6.0).round() as usize)
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
                    .into_iter()
                    .map(|(i, d)| match (i, d) {
                        (false, _) => None,
                        (true, d) => Some(*d),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn to_screen(&self) -> Screen {
        let dim = 0.8;
        let dim_max = (255.0 * dim) as u8;
        Screen {
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
                .collect::<Vec<Vec<Option<Rgb888>>>>()
                .clone(),
        } + Screen::from_float(self.q[31])
    }

    pub fn to_plot(&self) -> String {
        self.to_screen().to_string()
    }
}

impl fmt::Display for PriceQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", self.to_plot())
    }
}