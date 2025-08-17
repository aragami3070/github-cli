use octorust::{self, Client};
use std::error::Error;

use crate::cli_in::comment_command::CommentCommand;
use crate::cli_in::set_vars::{CommentTarget, Orders, Sorts};
use crate::cli_out::print_in_cli::{print_comments, print_review_comments};
use crate::git_utils::comments;
use crate::git_utils::repo_info::{Repo, RepoInfo, RepoName, RepoOwner};

pub async fn handle_comment_command(
    github_client: Client,
    subcommand: CommentCommand,
) -> Result<(), Box<dyn Error>> {
    match subcommand {
        CommentCommand::Create {
            owner,
            repo,
            number,
            body,
        } => {
            handle_create(github_client, owner, repo, number, body).await?;
            Ok(())
        }

        CommentCommand::GetAll { 
            owner,
            repo,
			number, target } => {
            handle_get_all(github_client, owner, repo, number, target).await?;
            Ok(())
        }

        CommentCommand::GetAllFromReview {
            number,
            sort,
            order,
        } => {
            handle_get_all_from_review(github_client, number, sort, order).await?;
            Ok(())
        }
    }
}

async fn handle_create(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    number: i64,
    body: String,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };
    let result = comments::create(&github_client, &repo_info, &number, &body).await?;

    println!("{result}");
    Ok(())
}

async fn handle_get_all(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    number: i64,
    target: CommentTarget,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };

    let result = comments::get_all(&github_client, &repo_info, &number).await?;

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
            .await?
        }
        CommentTarget::Issue => Vec::new(),
    };
    if !review_comments.is_empty() {
        print_review_comments(review_comments);
    }
    Ok(())
}

async fn handle_get_all_from_review(
    github_client: Client,
    number: i64,
    sort: Sorts,
    order: Orders,
) -> Result<(), Box<dyn Error>> {
    let repo_info: RepoInfo = RepoInfo::new(Repo::Current, None, None)?;

    let result =
        comments::get_all_from_review(&github_client, &repo_info, &number, sort.0, order.0).await?;

    print_review_comments(result);
    Ok(())
}
