use ::reqwest;
use clap::Parser;
use data::{Args, Repos};
use dmenu::create_dmenu_repo_list;
use exitfailure::ExitFailure;
use repo_helper::{create_repos_file, retrieve_from_cache};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

mod data;
mod dmenu;
mod repo_helper;

//TODO: Find a way to threat errors

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

// fn print_repos(all_repos: &Vec<Repos>) {
//     for repo in all_repos {
//         println!("Name: {}", repo.name);
//  println!("Repo Link: {}", repo.html_url);
//         println!("Private: {}", repo.private);
//     }
// }

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Args::parse();
    let api_key = args.api_key;

    let repos = retrieve_from_cache(&api_key).await?;
    create_dmenu_repo_list(&repos);

    Ok(())
}
