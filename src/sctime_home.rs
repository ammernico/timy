use std::fs;
use std::io;

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

    Ok(())
}
