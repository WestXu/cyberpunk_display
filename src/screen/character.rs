use std::ops;

use super::pixels_to_string::pixels_to_string;
use super::rgb::Rgb888;

pub struct Character {
    pub pixels: Vec<Vec<Option<Rgb888>>>,
}

impl Character {
    pub fn new(c: char) -> Character {
        let o = Some(Rgb888::new(255, 255, 255));
        let x = None;
        match c {
            ' ' => Character {
                pixels: vec![vec![], vec![], vec![], vec![], vec![]],
            },
            '|' => Character {
                pixels: vec![vec![x], vec![x], vec![x], vec![x], vec![x]],
            },
            '0' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![o, x, o],
                    vec![o, x, o],
                    vec![o, x, o],
                    vec![o, o, o],
                ],
            },
            '1' => Character {
                pixels: vec![
                    vec![x, o, x],
                    vec![o, o, x],
                    vec![x, o, x],
                    vec![x, o, x],
                    vec![o, o, o],
                ],
            },
            '2' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![x, x, o],
                    vec![o, o, o],
                    vec![o, x, x],
                    vec![o, o, o],
                ],
            },
            '3' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![x, x, o],
                    vec![o, o, o],
                    vec![x, x, o],
                    vec![o, o, o],
                ],
            },
            '4' => Character {
                pixels: vec![
                    vec![o, x, o],
                    vec![o, x, o],
                    vec![o, o, o],
                    vec![x, x, o],
                    vec![x, x, o],
                ],
            },
            '5' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![o, x, x],
                    vec![o, o, o],
                    vec![x, x, o],
                    vec![o, o, o],
                ],
            },
            '6' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![o, x, x],
                    vec![o, o, o],
                    vec![o, x, o],
                    vec![o, o, o],
                ],
            },
            '7' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![x, x, o],
                    vec![x, x, o],
                    vec![x, x, o],
                    vec![x, x, o],
                ],
            },
            '8' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![o, x, o],
                    vec![o, o, o],
                    vec![o, x, o],
                    vec![o, o, o],
                ],
            },
            '9' => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![o, x, o],
                    vec![o, o, o],
                    vec![x, x, o],
                    vec![o, o, o],
                ],
            },
            '.' => Character {
                pixels: vec![vec![x], vec![x], vec![x], vec![x], vec![o]],
            },
            _ => Character {
                pixels: vec![
                    vec![o, o, o],
                    vec![o, o, o],
                    vec![o, o, o],
                    vec![o, o, o],
                    vec![o, o, o],
                ],
            },
        }
    }
}

impl ToString for Character {
    fn to_string(&self) -> String {
        pixels_to_string(&self.pixels)
    }
}

fn concat_horizontal_of_2_vecs<T: Clone>(v1: Vec<Vec<T>>, v2: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert_eq!(v1.len(), v2.len());

    v1.into_iter()
        .zip(v2.into_iter())
        .into_iter()
        .map(|(row_of_v1, row_of_v2): (Vec<T>, Vec<T>)| {
            row_of_v1
                .iter()
                .cloned()
                .chain(row_of_v2.iter().cloned())
                .collect()
        })
        .collect()
}

impl ops::Add<Character> for Character {
    type Output = Character;

    fn add(self, _rhs: Character) -> Character {
        Character {
            pixels: concat_horizontal_of_2_vecs(
                concat_horizontal_of_2_vecs(self.pixels, Character::new('|').pixels),
                _rhs.pixels,
            ),
        }
    }
}
