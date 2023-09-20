use anyhow::anyhow;
use tracing::{debug, error};

mod print_markdown;
use print_markdown::print_markdown;
mod cli;
mod sctime_home;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        //.with_max_level(tracing::Level::INFO)
        .init();

    let app = cli::cli();
    let cli = app.get_matches();
    debug!("{:?}", cli.subcommand());

    match cli.subcommand() {
        Some(("print", _)) => {
            print_markdown();
        }
        Some(("sctime-home", _)) => {
            let _ = sctime_home::parse_sctime_home();
        }
        Some((other, _)) => {
            error!("Unknown subcommand: {}", other);
            error!("Use --help to find available subcommands");
            return Err(anyhow!("Unknown subcommand: {}", other));
        }
        None => {
            error!("No subcommand.");
            error!("Use --help to find available subcommands");
            return Err(anyhow!("No subcommand"));
        }
    }

    Ok(())
}
