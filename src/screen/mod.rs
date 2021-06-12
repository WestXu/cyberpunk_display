mod rgb;
use ansi_term::Colour;
pub use rgb::Rgb888;

pub struct Screen {
    pub pixels: Vec<Vec<Option<Rgb888>>>,
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        let (dot, blank) = ("██".to_string(), "  ".to_string());

        self.pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| match x {
                        Some(Rgb888 { r, g, b }) => Colour::RGB(*r, *g, *b).paint(&dot).to_string(),
                        _ => blank.clone(),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Screen {
    pub fn to_rgb565(&self) -> Vec<Vec<u16>> {
        self.pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| {
                        (match x {
                            Some(rgb888) => *rgb888,
                            _ => Rgb888::new(0, 0, 0),
                        })
                        .to_rgb565()
                    })
                    .collect()
            })
            .collect()
    }
}
