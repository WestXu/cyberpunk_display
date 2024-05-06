use clap::Parser;
use cyberpunk_display::awtrix;
use cyberpunk_display::matrix::BtcEthMatrix;
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
struct Matrix {}

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
    interval: Option<f64>, // in seconds, for screen to flip between matrix and builtin clock
}

#[cfg(feature = "nixie")]
#[derive(Parser, Debug)]
struct Nixie {
    #[clap(long)]
    serial_port: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Matrix(_) => {
            println!("\n\n\n\n\n\n\n\n");
            for screen in BtcEthMatrix::default() {
                println!("\x1b[8A{}", screen.to_string());
            }
        }
        SubCommand::Awtrix(a) => {
            let mut awtrix = awtrix::Awtrix::new(a.host, a.port);
            println!("\n\n\n\n\n\n\n\n");

            let start = std::time::Instant::now();
            let mut pause = false;

            for screen in BtcEthMatrix::default() {
                if a.print {
                    println!("\x1b[8A{}", screen.to_string());
                }
                match a.interval {
                    None => awtrix.plot(&screen.serialize()),
                    Some(interval) => {
                        let new_pause =
                            ((start.elapsed().as_secs_f64() / interval).round() as u64) % 2 == 0;
                        if !pause && new_pause {
                            awtrix.exit();
                        }
                        pause = new_pause;
                        if !pause {
                            awtrix.plot(&screen.serialize());
                        }
                    }
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
