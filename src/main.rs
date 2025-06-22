use clap::Parser;
use octorust::{self, auth::Credentials, Client};
use std::process;

mod cli_parse;
mod git_utils;

use crate::cli_parse::read_cli::set_state;
use crate::cli_parse::read_cli::Args;
use crate::cli_parse::read_cli::CliCommand;
use crate::git_utils::get_repo_info::get_current_repo;
use crate::git_utils::issues;

#[tokio::main]
async fn main() {
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
        CliCommand::IssuesList {
            creator,
            assignee,
            state,
            labels,
            numb_of_page,
            iss_on_page,
        } => {
            let inp_state = match set_state(&state) {
                Ok(res) => res,
                Err(message) => {
                    eprintln!("Error: {message}");
                    process::exit(1);
                }
            };

            let list_issues = issues::get_issues_list(
                &github_client,
                &repo_info,
                &creator,
                &assignee,
                &inp_state,
                &labels,
                &numb_of_page,
                &iss_on_page,
            )
            .await;
            println!(
                "{} {} Issues from {} page:",
                list_issues.len(),
                state,
                numb_of_page
            );
            println!();
            for issue in list_issues {
                println!("Issue{}: {};", issue.number, issue.title);
                println!("Body: {}", issue.body);
                println!();
            }
        }
        CliCommand::IssueCreate {
            title,
            body,
            assignees,
            labels,
        } => {
            let labels_list: Vec<String> = labels.split(",").map(|s| s.to_string()).collect();
			let mut assignees_list: Vec<String> = assignees.split(",").map(|s| s.to_string()).collect();

			let result = issues::create_issue(&github_client, &repo_info, &title, &body, &mut assignees_list, &labels_list).await;

			println!("{result}");
        }
    }
}
