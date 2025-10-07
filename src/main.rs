use clap::Parser;
use cyberpunk_display::awtrix;
use cyberpunk_display::matrix::{BtcEthMatrix, BtcTimeMatrix};
#[cfg(feature = "nixie")]
use cyberpunk_display::nixie;
#[cfg(feature = "nixie")]
use cyberpunk_display::ws_coin::WsCoin;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

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
    /// Minimum interval between updates (in milliseconds)
    #[clap(long)]
    min_interval: Option<u128>,
    /// Print matrix to terminal before sending to awtrix
    #[clap(long)]
    print: bool,
    #[clap(long)]
    time: bool, // show time instead of eth price
}

#[cfg(feature = "nixie")]
#[derive(Parser, Debug)]
struct Nixie {
    #[clap(short, long, default_value = "/dev/ttyUSB0")]
    serial_port: String,
    #[clap(short, long, default_value = "8")]
    brightness: u8,
}

#[tokio::main]
async fn main() {
    {
        let log_config = simplelog::ConfigBuilder::new()
            .set_time_format_rfc3339()
            .build();
        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Info,
                log_config.clone(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(LevelFilter::Debug, log_config, {
                let log_dir = PathBuf::from("tmp/logs");
                create_dir_all(&log_dir).unwrap();
                File::create(log_dir.join(format!(
                        "{}-{}.log",
                        chrono::Local::now().format("%Y%m%d%H%M%S"),
                        &uuid::Uuid::new_v4()
                            .as_hyphenated()
                            .encode_lower(&mut uuid::Uuid::encode_buffer())[..8]
                    )))
                .unwrap()
            }),
        ])
        .unwrap();
    }

    let opts: Opts = Opts::parse();

    log::info!("Starting application with {opts:?}");

    match opts.subcmd {
        SubCommand::Matrix(a) => {
            println!("\n\n\n\n\n\n\n\n");
            if a.time {
                let mut matrix = BtcTimeMatrix::default().await;
                loop {
                    let Some(screen) = matrix.gen_screen().await else {
                        continue;
                    };
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
            let mut awtrix = awtrix::Awtrix::new(a.host, a.port, a.min_interval);
            println!("\n\n\n\n\n\n\n\n");

            if a.time {
                let mut matrix = BtcTimeMatrix::default().await;
                loop {
                    let Some(screen) = matrix.gen_screen().await else {
                        continue;
                    };
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
            use futures::StreamExt as _;
            use rust_decimal_macros::dec;

            let mut nixie = nixie::Nixie::new(n.serial_port);
            nixie.set_brightness(n.brightness);
            let mut ws_coin = WsCoin::default().await;

            let mut latest_bytes = dec!(999999).into();
            loop {
                let Some(price) = ws_coin.next().await else {
                    continue;
                };
                log::info!("Received price: {:?}", price);
                let bytes = price.price.into();
                if bytes != latest_bytes {
                    latest_bytes = bytes;
                    nixie.send(bytes);
                }
            }
        }
    }
}
