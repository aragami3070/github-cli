use octorust::{self, Client};
use std::process;

use crate::cli_in::comment_command::CommentCommand;
use crate::git_utils::comments;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;

pub async fn handle_comment_command(github_client: Client, subcommand: CommentCommand) {
    match subcommand {
        CommentCommand::Create { number, body } => {
            handle_create(github_client, number, body).await;
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
