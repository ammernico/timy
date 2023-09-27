use std::fs;
use std::io;
use std::path::PathBuf;

use anyhow::anyhow;
use tracing::{debug, trace};

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

    //let entries = entries.into_iter();
    //let entries: Vec<PathBuf> = entries
    //    .iter()
    //    .filter_map(|e)| if e.exists() { Some(e) } else { None });
    //debug!("{:?}", entries);
    let entries: Vec<PathBuf> = entries
        .drain_fiter(|e| e.exists())
        .collect::<Vec<PathBuf>>();

    Ok(())
}
