use clap::Parser;
use cyberpunk_display::awtrix;
use cyberpunk_display::matrix::BtcEthMatrix;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Matrix(Matrix),
    Awtrix(Awtrix),
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
                if a.print {
                    println!("\x1b[8A{}", screen.to_string());
                }
                awtrix.plot(&screen.serialize())
            }
        }
    }
}
