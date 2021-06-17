use std::ops;

pub mod character;
mod pixels_to_string;
mod rgb;

use ordered_float::NotNan;

use character::Character;

use pixels_to_string::pixels_to_string;
pub use rgb::Rgb888;

pub struct Screen {
    pub pixels: Vec<Vec<Option<Rgb888>>>,
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        pixels_to_string(&self.pixels)
    }
}

impl Screen {
    pub fn from_chars(cs: Character) -> Self {
        let mut filled_cs = cs + Character::new(' '); // 右边加一列空像素
        while filled_cs.pixels[0].len() <= 31 {
            filled_cs = Character::new(' ') + filled_cs
        }

        let mut cut_pixels = filled_cs.pixels;

        while cut_pixels[0].len() >= 33 {
            cut_pixels = cut_pixels
                .into_iter()
                .map(|row| {
                    let mut new_row = row;
                    new_row.remove(0);
                    new_row
                })
                .collect()
        }

        let empty_row = vec![vec![None; 32]];
        let mut final_pixels = empty_row.clone();
        final_pixels.extend(cut_pixels);
        final_pixels.extend(empty_row.clone());
        final_pixels.extend(empty_row);

        assert_eq!(final_pixels.len(), 8);
        assert_eq!(final_pixels[0].len(), 32);
        Screen {
            pixels: final_pixels,
        }
    }
    pub fn from_float(p: NotNan<f64>) -> Self {
        let cs: Character = format!("{:.2}", p)
            .chars()
            .into_iter()
            .map(Character::new)
            .into_iter()
            .reduce(|a, b| a + b)
            .unwrap();

        Screen::from_chars(cs)
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
        .zip(v2.into_iter())
        .into_iter()
        .map(|(row_of_v1, row_of_v2): (Vec<Option<T>>, Vec<Option<T>>)| {
            row_of_v1
                .iter()
                .zip(&row_of_v2)
                .into_iter()
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
