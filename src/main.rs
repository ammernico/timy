use chrono::{Datelike, NaiveDate, Weekday};
use csv::ReaderBuilder;
use std::io;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, serde::Deserialize)]
struct Record {
    _id: String,
    _username: String,
    date: String,
    _account: String,
    sub_account: String,
    _time: Option<f32>,
    _billed_time: Option<f32>,
    comment: String,
}

fn parse_comment(comment: &String) -> String {
    let comment = comment.replace(", ", " \n    - ");
    let comment = comment.replace(",", " \n    - ");
    comment
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(io::stdin());

    let mut record_date = String::new();

    for result in rdr.deserialize() {
        let record: Record = match result {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse record: {e}");
                continue;
            }
        };

        if record.date == record_date {
            let comment = parse_comment(&record.comment);
            println!("    - {}", comment);
        } else {
            let chrono_date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d");
            let chrono_date = match chrono_date {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse time: {e}");
                    continue;
                }
            };

            print_record(&record, chrono_date);

            record_date = record.date;
        }
    }
    Ok(())
}

fn print_record(record: &Record, chrono_date: chrono::NaiveDate) {
    let chrono_date = chrono_date.weekday();
    let iso_day = chrono_weekday_translate(chrono_date);
    let comment = parse_comment(&record.comment);

    match record.sub_account.as_str() {
        "Schule" => {
            println!(
                "- {} (Schule) <!-- {} -->\n    - {}",
                iso_day, record.date, comment
            );
        }
        _ => {
            println!("- {} <!-- {} -->\n    - {}", iso_day, record.date, comment);
        }
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
