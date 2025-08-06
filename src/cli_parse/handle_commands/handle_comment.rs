use octorust::{self, Client};
use std::process;

use crate::cli_in::comment_command::CommentCommand;
use crate::cli_in::set_vars::{CommentTarget, Orders, Sorts};
use crate::cli_out::print_in_cli::{print_comments, print_review_comments};
use crate::git_utils::comments;
use crate::git_utils::repo_info::{Repo, RepoInfo};

pub async fn handle_comment_command(github_client: Client, subcommand: CommentCommand) {
    match subcommand {
        CommentCommand::Create { number, body } => {
            handle_create(github_client, number, body).await;
        }

        CommentCommand::GetAll { number, target } => {
            handle_get_all(github_client, number, target).await
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

async fn handle_get_all(github_client: Client, number: i64, target: CommentTarget) {
    let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
        Ok(rep) => rep,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let result = comments::get_all(&github_client, &repo_info, &number).await;

    print_comments(result);

    let review_comments = match target {
        CommentTarget::PullRequest => {
            comments::get_all_from_review(
                &github_client,
                &repo_info,
                &number,
                octorust::types::Sort::Created,
                octorust::types::Order::Asc,
            )
            .await
        }
        CommentTarget::Issue => Vec::new(),
    };
    if !review_comments.is_empty() {
        print_review_comments(review_comments);
    }
}

async fn handle_get_all_from_review(
    github_client: Client,
    number: i64,
    sort: Sorts,
    order: Orders,
) {
    let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
        Ok(rep) => rep,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let result =
        comments::get_all_from_review(&github_client, &repo_info, &number, sort.0, order.0).await;

    print_review_comments(result);
}
