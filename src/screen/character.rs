use std::ops;

use super::pixels_to_string::pixels_to_string;
use super::rgb::Rgb888;
use rust_decimal::prelude::*;

// Medium and Small fonts inspied by https://github.com/oidoid/mem
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Font {
    Large,
    Medium,
    Small,
}

pub struct Character {
    pub font: Font,
    pub pixels: Vec<Vec<Option<Rgb888>>>,
}

impl Character {
    pub fn new(c: char, font: Font) -> Character {
        let o = Some(Rgb888::new(255, 255, 255));
        let x = None;
        match font {
            Font::Large => match c {
                ' ' => Character {
                    font,
                    pixels: vec![vec![], vec![], vec![], vec![], vec![]],
                },
                '|' => Character {
                    font,
                    pixels: vec![vec![x], vec![x], vec![x], vec![x], vec![x]],
                },
                '0' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![o, x, o],
                        vec![o, x, o],
                        vec![o, x, o],
                        vec![o, o, o],
                    ],
                },
                '1' => Character {
                    font,
                    pixels: vec![
                        vec![x, o, x],
                        vec![o, o, x],
                        vec![x, o, x],
                        vec![x, o, x],
                        vec![o, o, o],
                    ],
                },
                '2' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![x, x, o],
                        vec![o, o, o],
                        vec![o, x, x],
                        vec![o, o, o],
                    ],
                },
                '3' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![x, x, o],
                        vec![o, o, o],
                        vec![x, x, o],
                        vec![o, o, o],
                    ],
                },
                '4' => Character {
                    font,
                    pixels: vec![
                        vec![o, x, o],
                        vec![o, x, o],
                        vec![o, o, o],
                        vec![x, x, o],
                        vec![x, x, o],
                    ],
                },
                '5' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![o, x, x],
                        vec![o, o, o],
                        vec![x, x, o],
                        vec![o, o, o],
                    ],
                },
                '6' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![o, x, x],
                        vec![o, o, o],
                        vec![o, x, o],
                        vec![o, o, o],
                    ],
                },
                '7' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![x, x, o],
                        vec![x, x, o],
                        vec![x, x, o],
                        vec![x, x, o],
                    ],
                },
                '8' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![o, x, o],
                        vec![o, o, o],
                        vec![o, x, o],
                        vec![o, o, o],
                    ],
                },
                '9' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![o, x, o],
                        vec![o, o, o],
                        vec![x, x, o],
                        vec![o, o, o],
                    ],
                },
                '.' => Character {
                    font,
                    pixels: vec![vec![x], vec![x], vec![x], vec![x], vec![o]],
                },
                ':' => Character {
                    font,
                    pixels: vec![vec![x], vec![o], vec![x], vec![o], vec![x]],
                },
                _ => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o],
                        vec![o, o, o],
                        vec![o, o, o],
                        vec![o, o, o],
                        vec![o, o, o],
                    ],
                },
            },
            Font::Medium => match c {
                ' ' => Character {
                    font,
                    pixels: vec![vec![], vec![], vec![], vec![]],
                },
                '|' => Character {
                    font,
                    pixels: vec![vec![x], vec![x], vec![x], vec![x]],
                },
                '0' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, x, o], //
                        vec![o, x, o], //
                        vec![o, o, o], //
                    ],
                },
                '1' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, x], //
                        vec![x, o, x], //
                        vec![x, o, x], //
                        vec![o, o, o], //
                    ],
                },
                '2' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![x, x, o], //
                        vec![o, o, x], //
                        vec![o, o, o], //
                    ],
                },
                '3' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, x], //
                        vec![x, o, o], //
                        vec![x, x, o], //
                        vec![o, o, o], //
                    ],
                },
                '4' => Character {
                    font,
                    pixels: vec![
                        vec![o, x, o], //
                        vec![o, x, o], //
                        vec![o, o, o], //
                        vec![x, x, o], //
                    ],
                },
                '5' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, x, x], //
                        vec![x, o, o], //
                        vec![o, o, o], //
                    ],
                },
                '6' => Character {
                    font,
                    pixels: vec![
                        vec![o, x, x], //
                        vec![o, o, o], //
                        vec![o, x, o], //
                        vec![o, o, o], //
                    ],
                },
                '7' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, x, o], //
                        vec![x, x, o], //
                        vec![x, x, o], //
                    ],
                },
                '8' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, x, o], //
                        vec![o, o, o], //
                        vec![o, o, o], //
                    ],
                },
                '9' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, x, o], //
                        vec![o, o, o], //
                        vec![x, x, o], //
                    ],
                },
                '.' => Character {
                    font,
                    pixels: vec![vec![x], vec![x], vec![x], vec![o]],
                },
                ':' => Character {
                    font,
                    pixels: vec![vec![x], vec![o], vec![x], vec![o]],
                },
                _ => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, o, o], //
                        vec![o, o, o], //
                        vec![o, o, o], //
                    ],
                },
            },
            Font::Small => match c {
                ' ' => Character {
                    font,
                    pixels: vec![vec![], vec![], vec![]],
                },
                '|' => Character {
                    font,
                    pixels: vec![vec![x], vec![x], vec![x]],
                },
                '0' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, x, o], //
                        vec![o, o, o], //
                    ],
                },
                '1' => Character {
                    font,
                    pixels: vec![
                        vec![x, o, x], //
                        vec![o, o, x], //
                        vec![x, o, x], //
                    ],
                },
                '2' => Character {
                    font,
                    pixels: vec![
                        vec![o, x, x], //
                        vec![x, o, x], //
                        vec![o, o, o], //
                    ],
                },
                '3' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![x, o, o], //
                        vec![o, o, o], //
                    ],
                },
                '4' => Character {
                    font,
                    pixels: vec![
                        vec![o, x, o], //
                        vec![o, o, o], //
                        vec![x, x, o], //
                    ],
                },
                '5' => Character {
                    font,
                    pixels: vec![
                        vec![x, o, o], //
                        vec![x, o, x], //
                        vec![o, o, x], //
                    ],
                },
                '6' => Character {
                    font,
                    pixels: vec![
                        vec![o, x, x], //
                        vec![o, o, o], //
                        vec![o, o, o], //
                    ],
                },
                '7' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![x, x, o], //
                        vec![x, x, o], //
                    ],
                },
                '8' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, o, o], //
                        vec![o, o, o], //
                    ],
                },
                '9' => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, o, o], //
                        vec![x, x, o], //
                    ],
                },
                '.' => Character {
                    font,
                    pixels: vec![vec![x], vec![x], vec![o]],
                },
                ':' => Character {
                    font,
                    pixels: vec![vec![o], vec![x], vec![o]],
                },
                _ => Character {
                    font,
                    pixels: vec![
                        vec![o, o, o], //
                        vec![o, o, o], //
                        vec![o, o, o], //
                    ],
                },
            },
        }
    }
    pub fn from_float(p: Decimal, font: Font) -> Self {
        format!("{:.2}", p)
            .chars()
            .map(|c| Character::new(c, font))
            .reduce(|a, b| a + b)
            .unwrap()
    }
    pub fn from_time(font: Font) -> Self {
        use chrono::Local;
        let dt = Local::now();

        dt.format("%H:%M:%S")
            .to_string()
            .chars()
            .map(|c| Character::new(c, font))
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", pixels_to_string(&self.pixels))
    }
}

fn concat_horizontal_of_2_vecs<T: Clone>(v1: Vec<Vec<T>>, v2: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert_eq!(v1.len(), v2.len());

    v1.into_iter()
        .zip(v2)
        .map(|(row_of_v1, row_of_v2): (Vec<T>, Vec<T>)| {
            row_of_v1.into_iter().chain(row_of_v2).collect()
        })
        .collect()
}

impl ops::Add<Character> for Character {
    type Output = Character;

    fn add(self, _rhs: Character) -> Character {
        assert_eq!(
            self.font, _rhs.font,
            "Can't add different fonts of {:?} and {:?}",
            self.font, _rhs.font
        );
        Character {
            font: self.font,
            pixels: concat_horizontal_of_2_vecs(
                concat_horizontal_of_2_vecs(self.pixels, Character::new('|', self.font).pixels),
                _rhs.pixels,
            ),
        }
    }
}
