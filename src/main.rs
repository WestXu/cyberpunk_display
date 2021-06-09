use rand::Rng;
use std::collections::vec_deque::VecDeque;

use std::fmt;

#[derive(Debug)]
struct PriceQueue {
    q: VecDeque<f64>,
}

impl PriceQueue {
    pub fn new() -> PriceQueue {
        let q = VecDeque::with_capacity(32);
        PriceQueue { q }
    }

    pub fn push(&mut self, p: f64) {
        if self.q.len() == 32 {
            self.q.pop_front();
        }
        self.q.push_back(p);
        while self.q.len() < 32 {
            self.q.push_back(p);
        }
    }
}

impl fmt::Display for PriceQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("-");
        for i in 1..self.q.len() {
            let p = self.q.get(i);
            let pre_p = self.q.get(i - 1);

            let char = if p == pre_p {
                '-'
            } else if p > pre_p {
                '↑'
            } else {
                '↓'
            };

            s.push(char);
        }
        write!(f, "{}", s)
    }
}

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
