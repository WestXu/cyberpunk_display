use ordered_float::NotNan;
use rand::Rng;

use cyberpunk_display::price_queue::PriceQueue;
use cyberpunk_display::screen::Screen;

fn main() {
    let mut rng = rand::thread_rng();

    let mut pq = PriceQueue::default();

    let mut p = NotNan::new(50000.0).unwrap();

    for _i in 1..20 {
        p += NotNan::new(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);
        println!("{:}", pq);
        println!("\n");
    }
}
