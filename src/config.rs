use std::fs::File;
use std::path::Path;

use ron::de::from_reader;

use self::errors::*;


pub struct Config {
    token: String,
}

impl Config {
    pub fn new(config_file: Path) -> Result<Self> {
        let mut f = File::open(config_file)?;
        from_reader(f)
    }
}
