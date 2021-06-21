use cyberpunk_display::price_queue::PriceQueue;
use cyberpunk_display::ws_coin::WsCoin;

fn main() {
    let mut pq = PriceQueue::default();
    for p in WsCoin::default() {
        pq.push(p);
        println!("{}", pq)
    }
}
