use anyhow::Result;
use std::collections::BTreeMap;
use std::fs::File;
use std::net::SocketAddr;
use std::path::Path;

pub static DEFAULT_CONFIG_PATH: &str = "config.yaml";

#[derive(Debug)]
pub struct Config {
    pub listen_address: SocketAddr,
    pub line_per_file: usize,
    pub file_per_folder: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            listen_address: "127.0.0.1:12345".parse().unwrap(),
            line_per_file: 1000,
            file_per_folder: 16,
        }
    }
}

pub fn new_from_file(file_path: &str) -> Result<Config> {
    let config_raw: BTreeMap<String, String> =
        serde_yaml::from_reader(File::open(Path::new(file_path))?)?;

    let mut config = Config::default();

    if let Some(listen_address) = config_raw.get("listen_address") {
        config.listen_address = listen_address.parse::<SocketAddr>()?;
    }

    if let Some(line_per_file) = config_raw.get("line_per_file") {
        config.line_per_file = line_per_file.parse::<usize>()?;
    }

    if let Some(file_per_folder) = config_raw.get("file_per_folder") {
        config.file_per_folder = file_per_folder.parse::<usize>()?;
    }

    Ok(config)
}
