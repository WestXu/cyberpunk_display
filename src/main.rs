use clap::Parser;
use cyberpunk_display::awtrix;
use cyberpunk_display::matrix::BtcTimeMatrix;
#[cfg(feature = "nixie")]
use cyberpunk_display::nixie;
#[cfg(feature = "nixie")]
use cyberpunk_display::ws_coin::WsCoin;
use futures::StreamExt as _;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::time::Duration;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

async fn drain_stream_or_wait<T>(
    stream: &mut (impl futures::Stream<Item = T> + Unpin),
) -> Option<T> {
    use futures::{FutureExt, StreamExt};

    let mut item: Option<T> = None;
    loop {
        let Some(next) = stream.next().now_or_never() else {
            // stream is drained
            if item.is_none() {
                // still empty, wait for next item
                return stream.next().await;
            }
            return item;
        };
        let Some(next) = next else {
            // stream is closed
            return item;
        };
        item = Some(next);
    }
}

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Matrix,
    Awtrix(Awtrix),
    #[cfg(feature = "nixie")]
    Nixie(Nixie),
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
        SubCommand::Matrix => {
            println!("\n\n\n\n\n\n\n\n");
            let mut matrix = BtcTimeMatrix::default().await;
            loop {
                let Some(screen) = matrix.next().await else {
                    continue;
                };
                println!("\x1b[8A{}", screen);
            }
        }
        SubCommand::Awtrix(a) => {
            let mut awtrix = awtrix::Awtrix::new(a.host, a.port, a.min_interval);
            println!("\n\n\n\n\n\n\n\n");

            let mut matrix = BtcTimeMatrix::default().await;
            loop {
                let screen = drain_stream_or_wait(&mut matrix).await.expect("closed");
                if a.print {
                    println!("\x1b[8A{}", screen);
                }
                awtrix.plot(&screen.serialize()).await;
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        }
        #[cfg(feature = "nixie")]
        SubCommand::Nixie(n) => {
            use cyberpunk_display::nixie::NixieMsg;

            let mut nixie = nixie::Nixie::new(n.serial_port);
            nixie.set_brightness(n.brightness);
            let mut ws_coin = WsCoin::default().await;

            let mut flip = false;
            loop {
                let price = drain_stream_or_wait(&mut ws_coin)
                    .await
                    .expect("WebSocket closed unexpectedly");

                log::debug!("Received price: {price:?}");
                let mut msg: NixieMsg = price.price.into();
                flip = !flip;
                if flip {
                    msg.flip_first_decimal_point()
                };
                nixie.send(msg).await;
            }
        }
    }
}
