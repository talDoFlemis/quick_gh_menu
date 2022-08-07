use anyhow::Result;
use clap::Parser;
use quick_gh_rust::{args_parser, dmenu, interactive};
use std::process;

//TODO: Find a way to threat errors

#[tokio::main]
async fn main() -> Result<()> {
    let args = args_parser::Cli::parse();

    match &args.command {
        args_parser::Commands::Dmenu => dmenu::run_dmenu().unwrap_or_else(|err| {
            eprintln!("{err:?}");
            eprintln!("Please, run a new setup with the setup SUBCOMMAND");
            process::exit(1);
        }),
        args_parser::Commands::Setup(data) => {
            if let Err(err) = quick_gh_rust::create_all(data).await {
                eprintln!("{err:?}");
                process::exit(1);
            }
        }
        args_parser::Commands::Interactive => interactive::run_interactively()
            .await
            .unwrap_or_else(|err| {
                eprintln!("{err:?}");
                process::exit(1);
            }),
    }

    Ok(())
}
