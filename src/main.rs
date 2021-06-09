use rand::Rng;

mod price_queue;
use self::price_queue::PriceQueue;

fn main() {
    let mut rng = rand::thread_rng();

    let mut pq = PriceQueue::new();

    let mut p = 100.0;

    for _i in 1..20 {
        p += rng.gen_range(-10.0..10.0);
        pq.push(p);
        println!("{:}", pq);
    }
}
