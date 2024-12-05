pub mod awtrix;
pub mod matrix;
#[cfg(feature = "nixie")]
pub mod nixie;
pub mod price_queue;
pub mod screen;
pub mod ws_coin;

#[test]
fn test_pq_screen() {
    // run with `cargo test test_pq_screen -- --nocapture`
    use rand::Rng;
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;
    use std::thread;
    use std::time::Duration;
    let mut rng = rand::thread_rng();

    let mut pq = price_queue::PriceQueue::default();

    let mut p = dec!(50000.0);

    println!("\n\n\n\n\n\n\n\n");
    for _i in 1..20 {
        p += Decimal::from_f64(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);
        println!("\x1b[8A{}", pq);
        thread::sleep(Duration::from_millis(100));
    }
}

#[test]
fn test_pq_screen_2_rows() {
    // run with `cargo test test_pq_screen_2_rows -- --nocapture`
    use rand::Rng;
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;
    use screen::character::{Character, Font};
    use screen::Screen;
    use std::thread;
    use std::time::Duration;
    let mut rng = rand::thread_rng();

    let mut pq = price_queue::PriceQueue::default();

    let mut p = dec!(50000.0);

    println!("\n\n\n\n\n\n\n\n");
    for _i in 1..20 {
        p += Decimal::from_f64(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);

        let mut screen = pq.to_screen(price_queue::PlotKind::FlatLine, false)
            + Screen::from_chars(Character::from_float(p, Font::Medium));
        let minor_cs = Character::from_float(p / dec!(100.0), Font::Small);
        screen.draw(&minor_cs.pixels, 32 - (minor_cs.pixels[0].len() + 1), 5);
        println!("\x1b[8A{}", screen);
        thread::sleep(Duration::from_millis(100));
    }
}
