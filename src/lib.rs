pub mod matrix;
pub mod price_queue;
pub mod py;
pub mod screen;
pub mod ws_coin;

#[test]
fn test_pq_screen() {
    // run with `cargo test test_pq_screen -- --nocapture`
    use ordered_float::NotNan;
    use rand::Rng;
    use std::thread;
    use std::time::Duration;
    let mut rng = rand::thread_rng();

    let mut pq = price_queue::PriceQueue::default();

    let mut p = NotNan::new(50000.0).unwrap();

    println!("\n\n\n\n\n\n\n\n");
    for _i in 1..20 {
        p += NotNan::new(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);
        println!("\x1b[8A{}", pq);
        thread::sleep(Duration::from_millis(100));
    }
}

#[test]
fn test_pq_screen_2_rows() {
    // run with `cargo test test_pq_screen_2_rows -- --nocapture`
    use ordered_float::NotNan;
    use rand::Rng;
    use screen::character::{Character, Font};
    use screen::Screen;
    use std::thread;
    use std::time::Duration;
    let mut rng = rand::thread_rng();

    let mut pq = price_queue::PriceQueue::default();

    let mut p = NotNan::new(50000.0).unwrap();

    println!("\n\n\n\n\n\n\n\n");
    for _i in 1..20 {
        p += NotNan::new(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);

        let mut screen = pq.to_screen(price_queue::PlotKind::FlatLine, false)
            + Screen::from_chars(Character::from_float(p, Font::Medium));
        let minor_cs = Character::from_float(p / 100.0, Font::Small);
        screen.draw(&minor_cs.pixels, 32 - (minor_cs.pixels[0].len() + 1), 5);
        println!("\x1b[8A{}", screen.to_string());
        thread::sleep(Duration::from_millis(100));
    }
}
