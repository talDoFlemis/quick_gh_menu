use clap::{Args, Parser, Subcommand};

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
}

#[derive(Args, Debug)]
pub struct Setup {
    ///Github API key for retrieving private repos
    pub api_key: String,
    #[clap(default_value_t = String::from("xdg-open"))]
    ///Browser to open the link
    pub browser: String,
    #[clap(default_value_t = false)]
    ///Case Sensitivity in dmenu
    pub case_insensitive: bool,
    #[clap(default_value_t = 0)]
    ///How many lines will be used in Dmenu
    pub lines: i32,
    #[clap(default_value_t = String::from("Choose a repository"))]
    ///Prompt to be displayed in Dmenu
    pub prompt_text: String,
}
