use clap::Parser;
use octorust::{self, auth::Credentials, Client};
use std::process;

mod cli_in;
mod cli_out;
mod cli_parse;
mod git_utils;

use crate::{cli_in::read_cli::Args, cli_parse::handle_cli::handle_cli_command};

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let github_token = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("Error: GITHUB_TOKEN enviroment variable not set");
            process::exit(1);
        }
    };

    let github_client: Client =
        Client::new("github-cli".to_string(), Credentials::Token(github_token))
            .expect("Failed to create Github client");

    match handle_cli_command(args, github_client).await {
        Ok(_) => {}
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
