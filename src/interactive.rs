use anyhow::{Context, Result};
use inquire::{Confirm, Password, Select, Text};

use crate::{browser, data::Repo, repo_helper};

#[derive(Default)]
struct Interactive {
    browser: browser::Browser,
    repos: Vec<Repo>,
}

pub async fn run_interactively() -> Result<()> {
    let browser_list = browser::Browser::get_all_browsers()?;

    let mut config = Interactive::default();

    let use_api = Confirm::new("Do you want to use your API key to fetch your repositories")
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
