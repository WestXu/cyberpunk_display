use cyberpunk_display::price_queue::PriceQueue;
use cyberpunk_display::ws_coin::WsCoin;

fn main() {
    let mut pq = PriceQueue::default();

    println!("\n\n\n\n\n\n\n\n");
    for p in WsCoin::default() {
        pq.push(p);
        println!("\x1b[8A{}", pq)
    }
}
