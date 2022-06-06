use dirs_next;
use exitfailure::ExitFailure;
use std::fs::{self, write, File, OpenOptions};
use std::io::{BufReader, BufWriter};

use crate::data::Repos;

//TODO: Add permission to execute to repo_list file
pub fn create_dmenu_repo_list(repos: &Vec<Repos>) {
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

pub fn create_dmenu_script() {
    let config_path = dirs_next::config_dir()
        .unwrap()
        .join("quickGHMenu/dmenu_conf");
}
