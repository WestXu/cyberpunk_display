use ansi_term::Colour;

#[derive(Clone, Copy)]
pub struct Rgb888 {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb888 {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb888 {
        Rgb888 { r, g, b }
    }

    pub fn to_term_rgb(&self) -> Colour {
        Colour::RGB(self.r, self.g, self.b)
    }

    pub fn to_rgb565(&self) -> u16 {
        let r5 = ((self.r >> 3) as u16) << 11;
        let g6 = ((self.g >> 2) as u16) << 5;
        let b5 = (self.b >> 3) as u16;

        r5 | g6 | b5
    }
}

#[test]
fn test_rgb888_to_rgb565() {
    assert_eq!(Rgb888::new(172, 10, 127).to_rgb565(), 43087);
}
