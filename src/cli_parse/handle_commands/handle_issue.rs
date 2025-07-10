use octorust::{self, Client};
use std::process;

use crate::cli_in::issue_command::IssueCommand;
use crate::cli_out::print_in_cli::print_issues;
use crate::git_utils::issues;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;

pub async fn handle_issue_command(github_client: Client, subcommand: IssueCommand) {
    match subcommand {
        IssueCommand::List {
            creator,
            assignee,
            state,
            labels,
            numb_of_page,
            iss_on_page,
        } => {
            let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                Ok(rep) => rep,
                Err(message) => {
                    eprintln!("Error: {message}");
                    process::exit(1);
                }
            };

            let list_issues = issues::get_list(
                &github_client,
                &repo_info,
                &creator,
                &assignee,
                &state.0,
                &labels,
                &numb_of_page,
                &iss_on_page,
            )
            .await;

            print_issues(list_issues, state, numb_of_page);
        }

        IssueCommand::Create {
            title,
            body,
            assignees,
            labels,
        } => {
            let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                Ok(rep) => rep,
                Err(message) => {
                    eprintln!("Error: {message}");
                    process::exit(1);
                }
            };
            let labels_list: Vec<String> = labels.split(",").map(|s| s.to_string()).collect();
            let assignees_list: Vec<String> = assignees.split(",").map(|s| s.to_string()).collect();

            let result = issues::create(
                &github_client,
                repo_info,
                &title,
                &body,
                &assignees_list,
                &labels_list,
            )
            .await;

            println!("{result}");
        }

        IssueCommand::Close { number, comment } => {
            let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                Ok(rep) => rep,
                Err(message) => {
                    eprintln!("Error: {message}");
                    process::exit(1);
                }
            };
            let result = issues::close(&github_client, repo_info, &number, &comment).await;

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
            let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                Ok(rep) => rep,
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

            let result = issues::update(
                &github_client,
                repo_info,
                &number,
                Some(title),
                Some(body),
                &assignees_list,
                &labels_list,
                &state.0,
            )
            .await;

            println!("{result}");
        }
    }
}
