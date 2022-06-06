use dirs_next;
use exitfailure::ExitFailure;
use std::fs::{self, write, File, OpenOptions};
use std::io::{BufReader, BufWriter};

use crate::data::Repos;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

async fn get_own_repos(api_key: &String) -> Result<Vec<Repos>, ExitFailure> {
    let url = "https://api.github.com/user/repos";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(AUTHORIZATION, format!("token {}", api_key))
        .header(USER_AGENT, "request")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    let repos: Vec<Repos> = resp.json().await?;
    // let repos: Vec<Repos> = match resp.status() {
    //     reqwest::StatusCode::OK => resp.json().await?,
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("Need to grab a new token");
    //     }
    //     other => {
    //         panic!("Uh oh! Something unexpected happened: {:?}", other);
    //     }
    // };
    Ok(repos)
}

pub async fn retrieve_from_cache(api_key: &String) -> Result<Vec<Repos>, ExitFailure> {
    let config_path = dirs_next::config_dir().unwrap();
    let repos_path = config_path.join("quickGHMenu/repos.json");

    if !repos_path.exists() {
        create_repos_file(&get_own_repos(api_key).await.unwrap());
    }

    let file = File::open(&repos_path)?;
    let repo_file = BufReader::new(file);

    let repos: Vec<Repos> = serde_json::from_reader(repo_file)?;

    Ok(repos)
}

pub fn create_repos_file(repos: &Vec<Repos>) {
    let config_path = dirs_next::config_dir().unwrap();
    let repos_path = config_path.join("quickGHMenu");

    //TODO: Change create to write func that will create a file and replace its content
    if !repos_path.exists() {
        fs::create_dir_all(&repos_path)
            .unwrap_or_else(|e| panic!("Error while trying to create dir: {}", e));
    }

    let file = File::create(repos_path.join("repos.json")).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &repos)
        .unwrap_or_else(|e| eprintln!("Error while trying to create a repo.json file: {}", e));
}
