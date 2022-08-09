use anyhow::{Context, Result};
use inquire::{Confirm, Password, Select, Text};

use crate::{browser, data::Repo, repo_helper, Config};

#[derive(Default)]
struct Interactive {
    browser: browser::Browser,
    repos: Vec<Repo>,
}

pub async fn run_interactively() -> Result<()> {
    let config = Config::retrieve_config();

    let interactive = if config.is_ok() {
        println!("Config file found!");
        if Confirm::new("Should we use the config file to fetch the repositories?")
            .with_default(true)
            .prompt()?
        {
            inter_with_cfg(config.unwrap()).await?
        } else {
            inter_without_cfg().await?
        }
    } else {
        println!("Couldn't find config file");
        println!("Starting to generate config file in interactive mode");
        inter_without_cfg().await?
    };

    select_repo(interactive)?;

    Ok(())
}

async fn inter_with_cfg(config: Config) -> Result<Interactive> {
    println!("Using the current setup");
    println!("Browser to open the links: {}", config.browser.browser_name);
    let repos = match config.method {
        crate::Method::ApiKey => {
            println!("Fetching repos with API key");
            repo_helper::get_repos_with_api(&config.key).await?
        }
        crate::Method::Username => {
            println!("Fetching repos with username");
            repo_helper::get_repos_with_user(&config.key).await?
        }
    };

    let config = Interactive {
        browser: config.browser,
        repos,
    };

    Ok(config)
}

async fn inter_without_cfg() -> Result<Interactive> {
    let browser_list = browser::Browser::get_all_browsers()?;

    let mut config = Interactive::default();

    let use_api = Confirm::new("Do you want to use your API key to fetch your repositories?")
        .with_help_message("This will enable you to see private repositories")
        .with_default(false)
        .prompt()?;

    config.repos = if use_api {
        let api_key = Password::new("Please provide your api key")
            .prompt()
            .context("Failed to resolve the API key")?;
        repo_helper::get_repos_with_api(&api_key).await?
    } else {
        let user = Text::new("Please provide your username")
            .prompt()
            .context("Failed to resolve username")?;
        repo_helper::get_repos_with_user(&user).await?
    };

    config.browser.browser_name = Select::new(
        "Please select your browser",
        browser_list
            .iter()
            .map(|browser| &browser.browser_name)
            .collect(),
    )
    .with_vim_mode(true)
    .prompt()?
    .to_owned();

    config.browser.command = if config.browser.browser_name == *String::from("Custom") {
        Text::new("Please put the custom browser command").prompt()?
    } else {
        browser_list
            .iter()
            .find(|bro| bro.browser_name == config.browser.browser_name)
            .unwrap()
            .command
            .to_owned()
    };

    Ok(config)
}

fn select_repo(config: Interactive) -> Result<()> {
    loop {
        let choosen_repo = Select::new(
            "Please select a repository from the list bellow",
            config.repos.iter().map(|repo| &repo.name).collect(),
        )
        .with_vim_mode(true)
        .prompt()?;

        match config.repos.iter().find(|&repo| &repo.name == choosen_repo) {
            Some(repo) => {
                browser::Browser::send_to_browser(&repo.html_url, config.browser.command.clone())?
            }
            None => eprintln!("No repository find"),
        };

        match Confirm::new("Do you want to choose another repository")
            .with_default(true)
            .prompt()?
        {
            true => continue,
            false => break,
        }
    }

    Ok(())
}
