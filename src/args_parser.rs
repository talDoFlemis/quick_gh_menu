use clap::{Args, Parser, Subcommand};

use crate::Method;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run dmenu with already fetched repos
    Dmenu,
    /// Create a config and repos files
    Setup(Setup),
    /// Run in interactive mode
    Interactive,
}

#[derive(Args, Debug)]
pub struct Setup {
    ///Use API key or username to fetch the repositories
    #[clap(arg_enum)]
    pub method: Method,
    ///Github API key or username depending on the method choosen
    pub key: String,
    #[clap(default_value_t = String::from("xdg-open"))]
    ///Browser to open the link
    pub browser: String,
    #[clap(default_value_t = true)]
    ///Case Sensitivity in dmenu
    pub case_insensitive: bool,
    #[clap(default_value_t = 0)]
    ///How many lines will be used in Dmenu
    pub lines: i32,
    #[clap(default_value_t = String::from("Choose a repository"))]
    ///Prompt to be displayed in Dmenu
    pub prompt_text: String,
}
