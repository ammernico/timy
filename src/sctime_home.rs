use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::anyhow;
use tracing::error;
use tracing::{debug, trace};
use xml::reader::XmlEvent;
use xml::EventReader;

struct XmlEntry {
    date: String,
    path: PathBuf,
}

pub fn parse_sctime_home() -> Result<(), anyhow::Error> {
    let mut sctime_dir = match home::home_dir() {
        Some(d) => {
            debug!("User home {}", d.display());
            d
        }
        None => {
            return Err(anyhow!("Failed to get user home dir"));
        }
    };
    sctime_dir.push(".sctime");

    let entries = fs::read_dir(sctime_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    trace!("{:?}", entries);

    let entries: Vec<PathBuf> = entries
        .into_iter()
        .filter_map(|e| if e.exists() { Some(e) } else { None })
        .collect();
    debug!("{:?}", entries);

    let dirs: Vec<_> = entries
        .iter()
        .filter_map(|d| {
            trace!("{:?}", d);
            let d: &PathBuf = match d.is_dir() {
                true => d,
                false => return None,
            };

            let name = d.file_name()?.to_str()?;
            if name.contains(".fail") {
                None
            } else {
                Some(d)
            }
        })
        .collect();

    let files: Vec<_> = entries
        .into_iter()
        .filter_map(|f| {
            debug!("{:?}", f);
            let name = f.file_name()?;
            let name = name.to_str()?;
            if name.contains("zeit") && name.contains(".xml") {
                Some(f)
            } else {
                None
            }
        })
        .collect();
    debug!("{:?}", files);

    /////////////////////////////////////////////////////////////////////////////////////////////////
    for f in &files {
        let file = File::open(f);
        let file = match file {
            Ok(f) => f,
            _ => {
                error!("Failed to open xml file {:?}", f);
                continue;
            }
        };

        let file = BufReader::new(file);
        let parser = EventReader::new(file);

        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    println!("{:spaces$}+{name}", "", spaces = depth * 2);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{:spaces$}-{name}", "", spaces = depth * 2);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                _ => {}
            }
        }
    }

    println!("files");
    for i in files {
        println!("{:?}", i);
    }
    Ok(())
}
