use crate::data::Repo;
use anyhow::{bail, Context, Ok, Result};
use dirs_next;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};

pub async fn get_repos_with_api(api_key: &str) -> Result<Vec<Repo>> {
    let url = "https://api.github.com/user/repos";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(AUTHORIZATION, format!("token {}", api_key))
        .header(USER_AGENT, "request")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .context("Failed to send request to github")?;

    let repos: Vec<Repo> = match resp.status() {
        reqwest::StatusCode::OK => resp
            .json()
            .await
            .context("Failed to deserialize the json retrived from github")?,

        reqwest::StatusCode::UNAUTHORIZED => bail!("Bad API key, need to grab another"),
        other => {
            bail!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(repos)
}

pub async fn get_repos_with_user(username: &str) -> Result<Vec<Repo>> {
    let url = format!("https://api.github.com/users/{}/repos", &username);
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(USER_AGENT, "request")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .context("Failed to send request to github")?;

    let repos: Vec<Repo> = match resp.status() {
        reqwest::StatusCode::OK => resp
            .json()
            .await
            .context("Failed to deserialize the json retrived from github")?,

        reqwest::StatusCode::UNAUTHORIZED => bail!("Bad API key, need to grab another"),
        other => {
            bail!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(repos)
}

pub fn create_repos_file(repos: &[Repo]) -> Result<()> {
    let config_path = dirs_next::config_dir().unwrap();
    let repos_path = config_path.join("quickGHMenu");

    //TODO: Change create to write func that will create a file and replace its content
    if !repos_path.exists() {
        fs::create_dir_all(&repos_path).context("Failed to create repositorys path")?;
    }

    let file = File::create(repos_path.join("repos.json")).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &repos).context("Failed to write repos.json")?;

    Ok(())
}

pub fn retrieve_from_file() -> Result<Vec<Repo>> {
    let config_path = dirs_next::config_dir().unwrap();
    let repos_path = config_path.join("quickGHMenu/repos.json");

    let file = File::open(&repos_path).with_context(|| format!("Failed to open {repos_path:?}"))?;
    let repo_file = BufReader::new(file);

    let repos: Vec<Repo> =
        serde_json::from_reader(repo_file).context("Failed to parse repos json")?;

    Ok(repos)
}
