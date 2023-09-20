use std::io;

use chrono::{Datelike, NaiveDate, Weekday};
use csv::ReaderBuilder;
use tracing::error;

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

pub fn print_markdown() {
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
            println!("    - {}", record.comment);
        } else {
            let chrono_date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d");
            let chrono_date = match chrono_date {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse time: {e}");
                    continue;
                }
            };
            let chrono_date = chrono_date.weekday();
            let iso_day = chrono_weekday_translate(chrono_date);
            if record.sub_account == "Schule" {
                println!(
                    "- {} (Schule) <!-- {} -->\n    - {}",
                    iso_day, record.date, record.comment
                );
            } else {
                println!(
                    "- {} <!-- {} -->\n    - {}",
                    iso_day, record.date, record.comment
                );
            }
            record_date = record.date;
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
