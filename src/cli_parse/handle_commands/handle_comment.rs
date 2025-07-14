use octorust::{self, Client};
use std::process;
// use std::str::FromStr;

use crate::cli_in::comment_command::CommentCommand;
use crate::cli_out::print_in_cli::print_comments;
use crate::git_utils::comments;
use crate::git_utils::repo_info::{Repo, RepoInfo};
// use crate::git_utils::repo_info::{RepoName, RepoOwner};

pub async fn handle_comment_command(github_client: Client, subcommand: CommentCommand) {
    match subcommand {
        CommentCommand::Create { number, body } => {
            handle_create(github_client, number, body).await;
        }

        CommentCommand::GetAll { number } => {
            handle_get_all_for_issue(github_client, number).await
        }
    };
}

async fn handle_create(github_client: Client, number: i64, body: String) {
    let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
        Ok(rep) => rep,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let result = comments::create(&github_client, &repo_info, &number, &body).await;

    println!("{result}");
}

async fn handle_get_all_for_issue(github_client: Client, number: i64) {
    let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
        Ok(rep) => rep,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let result = comments::get_all(&github_client, &repo_info, &number).await;

    print_comments(result);
}
