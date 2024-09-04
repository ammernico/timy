use anyhow::anyhow;
use chrono::Weekday;
use tracing::error;

pub mod latex;
pub mod markdown;

use self::latex::into_latex;
use self::markdown::into_markdown;

pub fn print(
    reader: csv::Reader<std::io::Stdin>,
    matches: clap::ArgMatches,
) -> Result<(), anyhow::Error> {
    match matches.subcommand() {
        Some(("markdown", _)) => into_markdown(reader),
        Some(("latex", _)) => into_latex(reader),
        Some((other, _)) => {
            error!("Unknown subcommand: {}", other);
            error!("Use --help to find available subcommands");
            Err(anyhow!("Unknown subcommand: {}", other))
        }
        None => panic!("No subcommand given"),
    }
}

fn chrono_weekday_translate(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => String::from("Montag"),
        Weekday::Tue => String::from("Dienstag"),
        Weekday::Wed => String::from("Mittwoch"),
        Weekday::Thu => String::from("Donnerstag"),
        Weekday::Fri => String::from("Freitag"),
        Weekday::Sat => String::from("Samstag"),
        Weekday::Sun => String::from("Sonntag"),
    }
}

fn parse_comment(comment: &str) -> String {
    let comment = comment.replace(", ", " \n    - ");
    comment.replace(',', " \n    - ")
}
