use std::ops;

pub mod character;
mod pixels_to_string;
pub mod rgb;

use ordered_float::NotNan;

use character::{Character, Font};

use pixels_to_string::pixels_to_string;
pub use rgb::Rgb888;

pub struct Screen {
    pub pixels: Vec<Vec<Option<Rgb888>>>,
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            pixels: vec![vec![None; 32]; 8],
        }
    }
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        pixels_to_string(&self.pixels)
    }
}

impl Screen {
    fn _is_in_screen(x: usize, y: usize) -> bool {
        (x <= 31) & (y <= 7)
    }
    pub fn draw(&mut self, pixels: &[Vec<Option<Rgb888>>], x0: usize, y0: usize) -> &Self {
        let height = pixels.len();
        let width = pixels[0].len();

        assert!(
            Self::_is_in_screen(x0, y0),
            "Starting point ({}, {}) is out of screen",
            x0,
            y0
        );

        for x in 0..width {
            #[allow(clippy::needless_range_loop)]
            for y in 0..height {
                if Self::_is_in_screen(x + x0, y + y0) {
                    self.pixels[y + y0][x + x0] = pixels[y][x];
                }
            }
        }

        self
    }
    pub fn from_chars(cs: Character) -> Self {
        let mut screen = Screen::default();
        screen.draw(&cs.pixels, 32 - (cs.pixels[0].len() + 1), 0);
        screen
    }
    pub fn from_float(p: NotNan<f64>) -> Self {
        Screen::from_chars(Character::from_float(p, Font::Large))
    }
    pub fn serialize(&self) -> Vec<u16> {
        self.pixels
            .iter()
            .flatten()
            .map(|x| {
                (match x {
                    Some(rgb888) => *rgb888,
                    _ => Rgb888::new(0, 0, 0),
                })
                .to_rgb565()
            })
            .collect()
    }
}

fn cover_v1_with_v2<T: Clone>(
    v1: Vec<Vec<Option<T>>>,
    v2: Vec<Vec<Option<T>>>,
) -> Vec<Vec<Option<T>>> {
    assert_eq!(v1.len(), v2.len());
    assert_eq!(v1[0].len(), v2[0].len());

    v1.into_iter()
        .zip(v2)
        .map(|(row_of_v1, row_of_v2): (Vec<Option<T>>, Vec<Option<T>>)| {
            row_of_v1
                .iter()
                .zip(&row_of_v2)
                .map(|(a, b)| match (a, b) {
                    (_, None) => a.clone(),
                    _ => b.clone(),
                })
                .collect()
        })
        .collect()
}

impl ops::Add<Screen> for Screen {
    type Output = Screen;

    fn add(self, _rhs: Screen) -> Screen {
        Screen {
            pixels: cover_v1_with_v2(self.pixels, _rhs.pixels),
        }
    }
}
