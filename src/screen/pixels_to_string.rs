pub use super::rgb::Rgb888;

pub fn pixels_to_string(pixels: &Vec<Vec<Option<Rgb888>>>) -> String {
    let (dot, blank) = ("██".to_string(), "  ".to_string());

    pixels
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| match x {
                    Some(rgb888) => rgb888.to_term_rgb().paint(&dot).to_string(),
                    _ => blank.clone(),
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
