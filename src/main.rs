use clap::ArgMatches;
use csv::ReaderBuilder;
use std::io;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
mod print;
mod record;

fn main() -> Result<(), anyhow::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::ERROR)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = cli::cli();
    let cli: ArgMatches = app.get_matches();
    debug!("{:?}", cli);

    let rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(io::stdin());

    let _ = print::print(rdr, cli);
    Ok(())
}
