use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod print_markdown;
use print_markdown::print_markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    print_markdown();

    Ok(())
}
