use clap::Parser;
use octorust::{self, auth::Credentials, Client};
use std::process;

mod cli_parse;
mod git_utils;

use crate::cli_parse::read_cli::Args;
use crate::cli_parse::read_cli::CliCommand;
use crate::git_utils::get_repo_info::get_current_repo;
use crate::git_utils::issues;

fn main() {
    let repo_info: String = match get_current_repo() {
        Ok(repo_url) => repo_url,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

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

    match args.command {
        CliCommand::IssuesList => {
            issues::get_issues_list(&github_client, &repo_info);
        }
    }
}
