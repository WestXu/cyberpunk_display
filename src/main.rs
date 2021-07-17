use cyberpunk_display::matrix::BtcEthMatrix;

fn main() {
    println!("\n\n\n\n\n\n\n\n");
    for screen in BtcEthMatrix::default() {
        println!("\x1b[8A{}", screen.to_string())
    }
}
