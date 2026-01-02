mod speed;
use clap::Parser;

/// A CLI tool for testing wifi download and upload speeds.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// List available servers sorted by distance
    #[arg(short, long)]
    list: bool,

    /// Specify a specific server ID to use
    #[arg(short, long)]
    server: Option<String>,

    /// Perform a download speed test
    #[arg(short, long)]
    down: bool,

    /// Perform an upload speed test
    #[arg(short, long)]
    up: bool,
}

fn main() {
    let args = CliArgs::parse();

    if args.list {
        if let Err(e) = speed::list_servers() {
            eprintln!("Error: {}", e);
        }
        return;
    }

    let run_downlaod = args.down || (!args.down && !args.up);
    let run_upload = args.up;

    if let Err(e) = speed::do_test(args.server, run_downlaod, run_upload) {
        eprintln!("Error: {}", e);
    }
}
