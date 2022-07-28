use std::{
    fs::File,
    io::{BufReader, BufWriter},
    process::Command,
};

use anyhow::{Context, Ok, Result};
use args_parser::Setup;
use serde_derive::{Deserialize, Serialize};

pub mod args_parser;
pub mod data;
pub mod dmenu;
pub mod errors;
pub mod repo_helper;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    dmenu: dmenu::DmenuSettings,
    api_key: String,
    browser: String,
}

impl Config {
    pub fn new(data: &Setup) -> Result<Self> {
        let api_key = (&data.api_key).to_string();
        let browser = (&data.browser).to_string();
        let config = Config {
            dmenu: dmenu::DmenuSettings::new(data),
            api_key,
            browser,
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

pub fn send_to_browser(url: String, browser: String) -> Result<()> {
    let mut openurl = Command::new(browser);
    openurl.arg(url);
    openurl.status().context("Failed to open the link")?;

    Ok(())
}

pub async fn create_all(data: &Setup) -> Result<()> {
    //TODO: Add a way to choose between or username or api_key as required
    // let username = match username {
    //     Some(user) => user,
    //     None => "none",
    // };

    Config::new(data)?;
    let repos = repo_helper::get_repos_with_api(&data.api_key).await?;
    repo_helper::create_repos_file(&repos)?;
    println!("Created with success the config file with theses settings: {data:#?}");

    Ok(())
}