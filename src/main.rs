use clap::Parser;
use cyberpunk_display::awtrix;
use cyberpunk_display::matrix::{BtcEthMatrix, BtcTimeMatrix};
#[cfg(feature = "nixie")]
use cyberpunk_display::nixie;
#[cfg(feature = "nixie")]
use cyberpunk_display::ws_coin::WsCoin;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Matrix(Matrix),
    Awtrix(Awtrix),
    #[cfg(feature = "nixie")]
    Nixie(Nixie),
}

#[derive(Parser, Debug)]
struct Matrix {
    #[clap(long)]
    time: bool, // show time instead of eth price
}

#[derive(Parser, Debug)]
struct Awtrix {
    #[clap(long, default_value = "localhost")]
    host: String,
    #[clap(long, default_value = "7000")]
    port: u16,
    /// Print matrix to terminal before sending to awtrix
    #[clap(long)]
    print: bool,
    #[clap(long)]
    time: bool, // show time instead of eth price
}

#[cfg(feature = "nixie")]
#[derive(Parser, Debug)]
struct Nixie {
    #[clap(long)]
    serial_port: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Matrix(a) => {
            println!("\n\n\n\n\n\n\n\n");
            if a.time {
                let mut matrix = BtcTimeMatrix::default().await;
                loop {
                    let screen = matrix.gen_screen().await;
                    println!("\x1b[8A{}", screen);
                }
            } else {
                let mut matrix = BtcEthMatrix::default().await;
                loop {
                    let screen = matrix.gen_screen().await;
                    println!("\x1b[8A{}", screen);
                }
            }
        }
        SubCommand::Awtrix(a) => {
            let mut awtrix = awtrix::Awtrix::new(a.host, a.port);
            println!("\n\n\n\n\n\n\n\n");

            if a.time {
                let mut matrix = BtcTimeMatrix::default().await;
                loop {
                    let screen = matrix.gen_screen().await;
                    if a.print {
                        println!("\x1b[8A{}", screen);
                    }
                    awtrix.plot(&screen.serialize()).await
                }
            } else {
                let mut matrix = BtcEthMatrix::default().await;
                loop {
                    let screen = matrix.gen_screen().await;
                    if a.print {
                        println!("\x1b[8A{}", screen);
                    }
                    awtrix.plot(&screen.serialize()).await
                }
            }
        }
        #[cfg(feature = "nixie")]
        SubCommand::Nixie(n) => {
            let mut nixie = nixie::Nixie::new(n.serial_port);
            nixie.set_brightness(8);
            let ws_coin = WsCoin::default();
            let mut lastest_price = 99999.9;
            for price in ws_coin {
                let p = *price.price.as_f32();
                if p != lastest_price {
                    lastest_price = p;
                }
                nixie.send(lastest_price)
            }
        }
    }
}
