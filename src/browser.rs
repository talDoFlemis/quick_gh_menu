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
        let mut mime_cache = file
            .lines()
            .find(|&line| line.contains("x-scheme-handler/https"))
            .unwrap()
            .strip_prefix("x-scheme-handler/https=")
            .unwrap()
            .chars();

        mime_cache.next_back();
        let apps = mime_cache.as_str().split(';');

        for app in apps {
            let app_entry = fs::read_to_string(format!("/usr/share/applications/{app}"))
                .with_context(|| format!("Couldn't read {app}"))?;

            let browser_name = app_entry
                .lines()
                .find(|line| line.contains("Name="))
                .unwrap()
                .strip_prefix("Name=")
                .unwrap()
                .to_string();

            let command = app_entry
                .lines()
                .find(|line| line.contains("Exec="))
                .unwrap()
                .strip_prefix("Exec=")
                .unwrap()
                .to_string();

            browsers.push(Browser {
                browser_name,
                command,
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
