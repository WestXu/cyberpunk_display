use std::collections::vec_deque::VecDeque;
use std::fmt;

use ansi_term::Colour::{Blue, Green, Red};
use ordered_float::NotNan;

mod rgb;
use rgb::Rgb888;

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

    pub fn to_up_down(&self) -> Vec<Direction> {
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

    pub fn to_plot(&self) -> String {
        let (dot, blank) = ("██".to_string(), "  ".to_string());
        let (blue, green, red) = (
            Blue.paint(&dot).to_string(),
            Green.paint(&dot).to_string(),
            Red.paint(&dot).to_string(),
        );

        self.to_2d_direction_array()
            .into_iter()
            .map(|row| {
                row.iter()
                    .map(|x| match x {
                        Some(Direction::Flat) => &blue[..],
                        Some(Direction::Up) => &green[..],
                        Some(Direction::Down) => &red[..],
                        _ => &blank[..],
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn to_rgb888(&self) -> Vec<Vec<Rgb888>> {
        self.to_2d_direction_array()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| match x {
                        Some(Direction::Flat) => Rgb888::new(0, 0, 255),
                        Some(Direction::Up) => Rgb888::new(0, 255, 0),
                        Some(Direction::Down) => Rgb888::new(255, 0, 0),
                        _ => Rgb888::new(0, 0, 0),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn to_rgb565(&self) -> Vec<Vec<u16>> {
        self.to_rgb888()
            .iter()
            .map(|row| row.iter().map(|rgb888| rgb888.to_rgb565()).collect())
            .collect()
    }
}

impl fmt::Display for PriceQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", self.to_plot())
    }
}
