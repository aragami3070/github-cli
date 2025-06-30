use clap::Parser;
use octorust::{self, auth::Credentials, Client};
use std::process;

mod cli_parse;
mod git_utils;

use crate::cli_parse::set_vars::set_issues_list_state;
use crate::cli_parse::set_vars::set_option_string;
use crate::cli_parse::set_vars::set_state;
use crate::cli_parse::read_cli::Args;
use crate::cli_parse::read_cli::CliCommand;
use crate::cli_parse::read_cli::IssueCommand;
use crate::git_utils::common::create_comment;
use crate::git_utils::get_repo_info::get_current_repo;
use crate::git_utils::issues;
use crate::git_utils::issues::update_issue;

fn get_repo() -> String {
    return match get_current_repo() {
        Ok(repo_url) => repo_url,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };
}

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

    match args.command {
        CliCommand::Issue { subcommand } => match subcommand {
            IssueCommand::List {
                creator,
                assignee,
                state,
                labels,
                numb_of_page,
                iss_on_page,
            } => {
                let repo_info: String = get_repo();
                let inp_state = match set_issues_list_state(&state) {
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

            IssueCommand::Create {
                title,
                body,
                assignees,
                labels,
            } => {
                let repo_info: String = get_repo();
                let labels_list: Vec<String> = labels.split(",").map(|s| s.to_string()).collect();
                let assignees_list: Vec<String> =
                    assignees.split(",").map(|s| s.to_string()).collect();

                let result = issues::create_issue(
                    &github_client,
                    &repo_info,
                    &title,
                    &body,
                    &assignees_list,
                    &labels_list,
                )
                .await;

                println!("{result}");
            }

            IssueCommand::Close { number, comment } => {
                let repo_info: String = get_repo();
                let result =
                    issues::close_issue(&github_client, &repo_info, &number, &comment).await;

                println!("{result}");
            }

            IssueCommand::Update {
                number,
                title,
                body,
                assignees,
                state,
                labels,
            } => {
                let repo_info: String = get_repo();
                let new_state = match set_state(&state) {
                    Ok(s) => s,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let labels_list: Vec<String> = match labels {
                    Some(l) => l.split(",").map(|s| s.to_string()).collect(),
                    None => Vec::new(),
                };
                let assignees_list: Vec<String> = match assignees {
                    Some(a) => a.split(",").map(|s| s.to_string()).collect(),
                    None => Vec::new(),
                };

                let new_body: Option<&String> = set_option_string(&body);
                let new_title: Option<&String> = set_option_string(&title);

                let result = update_issue(
                    &github_client,
                    &repo_info,
                    &number,
                    new_title,
                    new_body,
                    &assignees_list,
                    &labels_list,
                    &new_state,
                )
                .await;

                println!("{result}");
            }
        },

        CliCommand::CreateComment { number, body } => {
            let repo_info: String = get_repo();
            let result = create_comment(&github_client, &repo_info, &number, &body).await;

            println!("{result}");
        }
    }
}
