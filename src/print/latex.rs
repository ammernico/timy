use chrono::{Datelike, NaiveDate};

use crate::print::chrono_weekday_translate;
use crate::record::Record;

struct LastLatexRecord {
    // to check if the day changed
    date: String,
    comment: String,
    sub_account: String,

    // the iso week of the current record to increment the week
    iso_week: u32,
    // counter for the current week
    week: u32,
}

pub fn into_latex(mut reader: csv::Reader<std::io::Stdin>) -> Result<(), anyhow::Error> {
    let mut record_iter = reader.deserialize::<Record>().peekable();

    let first_record = record_iter.next().unwrap()?;

    let mut last_record = LastLatexRecord {
        date: first_record.date,
        comment: first_record.comment,
        sub_account: first_record.sub_account,
        iso_week: 0,
        week: 0,
    };

    while let Some(record) = record_iter.next() {
        let record: Record = match record {
            Ok(r) => r,
            Err(e) => {
                panic!("Failed to parse record: {e}");
            }
        };

        if record.date == last_record.date {
            if last_record.comment == *"" {
                last_record.comment += &record.comment;
            } else {
                last_record.comment += &(", ".to_string() + &record.comment);
            }
        } else {
            print_latex_record(&mut last_record);

            last_record = LastLatexRecord {
                date: record.date,
                comment: record.comment,
                sub_account: record.sub_account,
                iso_week: last_record.iso_week,
                week: last_record.week,
            };
        }

        if record_iter.peek().is_none() {
            print_latex_record(&mut last_record)
        }
    }

    Ok(())
}

fn print_latex_record(last_record: &mut LastLatexRecord) {
    let chrono_date = NaiveDate::parse_from_str(&last_record.date, "%Y-%m-%d");
    let chrono_date = match chrono_date {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed to parse time: {e}");
        }
    };

    {
        // update counter for week number
        let chrono_iso_week = chrono_date.iso_week();
        let chrono_iso_week = chrono_iso_week.week();

        if last_record.iso_week != chrono_iso_week {
            last_record.iso_week = chrono_iso_week;

            last_record.week += 1;
            println!("% Woche: {}", last_record.week);
        }
    }

    let day = chrono_date.weekday();
    let day = chrono_weekday_translate(day);

    println!(
        "% {}\n\\newcommand{{\\{}}}\n{{{}: {}}}",
        last_record.date,
        day.to_lowercase(),
        last_record.sub_account,
        last_record.comment
    );
}
