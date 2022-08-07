use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use anyhow::{Context, Ok, Result};
use args_parser::Setup;
use serde_derive::{Deserialize, Serialize};

pub mod args_parser;
pub mod browser;
pub mod data;
pub mod dmenu;
pub mod errors;
pub mod interactive;
pub mod repo_helper;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    dmenu: dmenu::DmenuSettings,
    api_key: String,
    browser: browser::Browser,
}

impl Config {
    pub fn new(data: &Setup) -> Result<Self> {
        let api_key = (&data.api_key).to_string();
        let browser_command = (&data.browser).to_string();
        let config = Config {
            dmenu: dmenu::DmenuSettings::new(data),
            api_key,
            browser: browser::Browser {
                browser_name: String::from("custom"),
                command: browser_command,
            },
        };

        let config_path = dirs_next::config_dir().unwrap().join("quickGHMenu");

        let file = File::create(config_path.join("config.json"))?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &config)
            .context("Failed to write the config file ")?;

        Ok(config)
    }

    fn retrieve_config() -> Result<Self> {
        let config_path = dirs_next::config_dir()
            .unwrap()
            .join("quickGHMenu/config.json");

        let file = File::open(&config_path)
            .with_context(|| format!("Failed to read the config file {:?}", &config_path))?;
        let config_file = BufReader::new(file);

        let config: Config = serde_json::from_reader(config_file)
            .context("Failed to deserialize the config file")?;

        Ok(config)
    }
}

pub async fn create_all(data: &Setup) -> Result<()> {
    Config::new(data)?;
    let repos = repo_helper::get_repos_with_api(&data.api_key).await?;
    repo_helper::create_repos_file(&repos)?;
    println!("Created with success the config file with theses settings: {data:#?}");

    Ok(())
}
