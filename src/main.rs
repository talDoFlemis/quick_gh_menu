use clap::Parser;
use data::Args;
use dmenu::create_dmenu_repo_list;
use exitfailure::ExitFailure;
use repo_helper::retrieve_from_cache;

mod data;
mod dmenu;
mod repo_helper;

//TODO: Find a way to threat errors

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Args::parse();
    let api_key = args.api_key;

    let repos = retrieve_from_cache(&api_key).await?;
    create_dmenu_repo_list(&repos);

    Ok(())
}
