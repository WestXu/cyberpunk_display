use ordered_float::NotNan;
use rand::Rng;

use cyberpunk_display::price_queue::PriceQueue;
use cyberpunk_display::screen::character::Character;

fn main() {
    let mut rng = rand::thread_rng();

    let mut pq = PriceQueue::default();

    let mut p = NotNan::new(100.0).unwrap();

    for c in String::from(" 0123456789.").chars() {
        println!("{} \n", Character::new(c).to_string())
    }

    println!(
        "{} \n",
        String::from(" 0123456789.")
            .chars()
            .into_iter()
            .map(Character::new)
            .into_iter()
            .reduce(|a, b| a + b)
            .unwrap()
            .to_string()
    );

    for _i in 1..20 {
        p += NotNan::new(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);
        println!("{:}", pq);
        println!("\n");
    }
}
