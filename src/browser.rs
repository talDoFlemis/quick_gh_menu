use anyhow::{Context, Result};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::{fs, process::Command};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Browser {
    pub browser_name: String,
    pub command: String,
}

impl Browser {
    pub fn get_all_browsers() -> Result<Vec<Self>> {
        let mut browsers: Vec<Self> = Vec::new();
        let file = fs::read_to_string("/usr/share/applications/mimeinfo.cache")?;
        let find_x_scheme = Regex::new(r"x-scheme-handler/https=.*").unwrap();
        let x_scheme_https = find_x_scheme.find(&file).unwrap();
        let cap_programs = Regex::new(r"([^=;]*.desktop)").unwrap();

        for cap in cap_programs.captures_iter(x_scheme_https.as_str()) {
            let app_entry = fs::read_to_string(format!("/usr/share/applications/{}", &cap[0]))
                .with_context(|| format!("Couldn't read {}", &cap[0]))?;

            let find_browser_name = Regex::new(r"Name=(.*)").unwrap();
            let browser_name = find_browser_name.captures(&app_entry).unwrap();

            let find_browser_command = Regex::new(r"Exec=(.*)").unwrap();
            let command = find_browser_command.captures(&app_entry).unwrap();

            browsers.push(Browser {
                browser_name: browser_name[1].to_string(),
                command: command[1].to_string(),
            })
        }

        browsers.push(Browser {
            browser_name: String::from("Custom"),
            command: String::from("custom command"),
        });

        Ok(browsers)
    }

    pub fn send_to_browser(url: &String, browser: String) -> Result<()> {
        let re = Regex::new(r"\s.*").unwrap();
        let parsed_browser = re.replace(browser.as_str(), "");
        let mut openurl = Command::new(parsed_browser.to_string());
        openurl.arg(url);
        openurl.status().context("Failed to open the link")?;

        Ok(())
    }
}
