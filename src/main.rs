use clap::{AppSettings, Clap};
use cyberpunk_display::awtrix;
use cyberpunk_display::matrix::BtcEthMatrix;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Matrix(Matrix),
    Awtrix(Awtrix),
}

#[derive(Clap)]
struct Matrix {}

#[derive(Clap)]
struct Awtrix {
    #[clap(short, long, default_value = "localhost")]
    host: String,
    #[clap(short, long, default_value = "7000")]
    port: u16,
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
            for screen in BtcEthMatrix::default() {
                println!("\x1b[8A{}", screen.to_string());
                awtrix.plot(&screen.serialize())
            }
        }
    }
}
