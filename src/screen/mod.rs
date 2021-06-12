pub mod character;
mod pixels_to_string;
mod rgb;

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
