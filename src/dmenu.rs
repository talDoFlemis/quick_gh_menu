use crate::args_parser::Setup;
use crate::data::Repo;
use crate::{repo_helper, send_to_browser, Config};
use anyhow::{Context, Result};
use dirs_next;
use serde_derive::{Deserialize, Serialize};
use std::fs::write;
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DmenuSettings {
    case_insensitive: bool,
    prompt_text: String,
    lines: i32,
}

impl DmenuSettings {
    pub fn new(data: &Setup) -> Self {
        DmenuSettings {
            case_insensitive: data.case_insensitive,
            lines: data.lines,
            prompt_text: data.prompt_text.clone(),
        }
    }
    //TODO: Colocar lifetime no bagulho
    fn args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if self.case_insensitive {
            args.push("-i".to_string());
        }

        if self.lines != 0 {
            args.push("-l".to_string());
            args.push(self.lines.to_string());
        }

        args.push("-p".to_string());
        args.push(self.prompt_text.clone());

        args
    }
}

//TODO: Add permission to execute to repo_list file
pub fn create_dmenu_repo_list(repos: &Vec<Repo>) {
    let config_path = dirs_next::config_dir()
        .unwrap()
        .join("quickGHMenu/repo_list");

    let mut repo_list: String =
        "#!/bin/bash\n#Repo list\ndeclare -A github_repo_list\n\n".to_owned();
    for repo in repos {
        let line: String =
            format!("github_repo_list[{}]=\"{}\"\n", repo.name, repo.html_url).to_owned();
        repo_list.push_str(&line);
    }

    write(&config_path, repo_list).unwrap(); // for repo in repos {}
}

pub fn run_dmenu() -> Result<()> {
    let repos = repo_helper::retrieve_from_file().unwrap();
    let mut names_pipe = String::new();
    let config = Config::retrieve_config()?;
    for repo in &repos {
        names_pipe.push_str(format!("{}\n", repo.name).as_str());
    }

    let dmenu = Command::new("dmenu")
        .args(&config.dmenu.args())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Couldn't spawn dmenu")?;

    dmenu
        .stdin
        .unwrap()
        .write_all(names_pipe.as_bytes())
        .context("Couldn't write to dmenu stdin")?;

    let mut choice = String::new();
    dmenu
        .stdout
        .unwrap()
        .read_to_string(&mut choice)
        .context("Couldn't read dmenu stdout")?;

    let mut url = String::new();

    for repo in repos {
        if repo.name == choice.trim_end() {
            url = repo.html_url;
        }
    }

    send_to_browser(url, config.browser)?;

    Ok(())
}
