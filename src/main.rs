use ::reqwest;
use clap::Parser;
use exitfailure::ExitFailure;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_derive::{Deserialize, Serialize};

#[derive(Parser)]
struct Args {
    username: String,
    api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
    private: bool,
    html_url: String,
}

async fn get_own_repos(api_key: &String) -> Result<(), ExitFailure> {
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

    match resp.status() {
        reqwest::StatusCode::OK => {
            match resp.json::<Vec<Response>>().await {
                Ok(parsed) => print_repos(parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected. "),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
    Ok(())
}

fn print_repos(all_repos: Vec<Response>) {
    for repo in all_repos {
        println!("Name: {}", repo.name);
        println!("Repo Link: {}", repo.html_url);
        println!("Private: {}", repo.private);
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Args::parse();
    let username = args.username;
    let api_key = args.api_key;

    get_own_repos(&api_key).await?;

    Ok(())
}
