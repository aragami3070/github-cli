use std::process;

use octorust::types::{
    IssueComment, Order, PullRequestReviewComment, PullsUpdateReviewRequest, Sort,
};
use octorust::Client;

use crate::git_utils::repo_info::RepoInfo;

pub async fn create(
    github_client: &Client,
    repo_info: &RepoInfo,
    issue_number: &i64,
    body: &String,
) -> String {
    let request = PullsUpdateReviewRequest { body: body.clone() };

    let comment = github_client
        .issues()
        .create_comment(
            &repo_info.get_owner().trim(),
            &repo_info.get_name().trim(),
            issue_number.clone(),
            &request,
        )
        .await;

    return match comment {
        Ok(_) => "Comment create successed".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

// Get all Comments for issue/pull request without review comments
pub async fn get_all(
    github_client: &Client,
    repo_info: &RepoInfo,
    number: &i64,
) -> Vec<IssueComment> {
    let list_comments = github_client
        .issues()
        .list_all_comments(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            number.clone(),
            None,
        )
        .await;

    return match list_comments {
        Ok(c) => c.body,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
pub async fn get_all_from_review(
    github_client: &Client,
    repo_info: &RepoInfo,
    number: &i64,
    sort: Sort,
    direction: Order,
) -> Vec<PullRequestReviewComment> {
    let list_comments = github_client
        .pulls()
        .list_all_review_comments(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            number.clone(),
            sort,
            direction,
            None,
        )
        .await;

    return match list_comments {
        Ok(c) => c.body,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
