use chrono::{Datelike, NaiveDate};
use tracing::error;

use crate::print::{chrono_weekday_translate, parse_comment};
use crate::record::Record;

pub fn into_markdown(mut reader: csv::Reader<std::io::Stdin>) -> Result<(), anyhow::Error> {
    let mut record_date = String::new();

    let mut week_n: u32 = 1;
    let mut iso_week: u32 = 0;

    for result in reader.deserialize() {
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

            print_markdown_record(&record, chrono_date, &mut week_n, &mut iso_week);

            record_date = record.date;
        }
    }

    Ok(())
}

fn print_markdown_record(
    record: &Record,
    chrono_date: chrono::NaiveDate,
    last_week_n: &mut u32,
    last_iso_week: &mut u32,
) {
    {
        // counter for week number
        let chrono_iso_week = chrono_date.iso_week();
        let chrono_iso_week = chrono_iso_week.week();
        if chrono_iso_week != *last_iso_week {
            *last_iso_week = chrono_iso_week;

            println!("<!-- {} -->", last_week_n);
            *last_week_n += 1;
        }
    }

    let chrono_weekday = chrono_date.weekday();
    let iso_day = chrono_weekday_translate(chrono_weekday);
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
