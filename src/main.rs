use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut p = 50000.0;

    for _i in 1..20 {
        p += rng.gen_range(-1000.0..1000.0);
        println!("{:2}", p);
    }
}
