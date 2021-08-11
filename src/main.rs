use cyberpunk_display::awtrix::Awtrix;
use cyberpunk_display::matrix::BtcEthMatrix;

fn main() {
    let mut awtrix = Awtrix::new("localhost".to_string(), 7000);
    println!("\n\n\n\n\n\n\n\n");
    for screen in BtcEthMatrix::default() {
        println!("\x1b[8A{}", screen.to_string());
        awtrix.plot(&screen.serialize())
    }
}
