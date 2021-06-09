use ordered_float::NotNan;
use rand::Rng;

mod price_queue;
use self::price_queue::PriceQueue;

fn main() {
    let mut rng = rand::thread_rng();

    let mut pq = PriceQueue::new();

    let mut p = NotNan::new(100.0).unwrap();

    for _i in 1..20 {
        p += NotNan::new(rng.gen_range(-10.0..10.0)).unwrap();
        pq.push(p);
        println!("{:}", pq);
    }
}
