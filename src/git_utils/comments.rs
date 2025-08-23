use std::error::Error;

use octorust::types::{
    IssueComment, Order, PullRequestReviewComment, PullsUpdateReviewRequest, Sort,
};
use octorust::Client;

use crate::git_utils::repo_info::RepoInfo;

pub async fn create(
    github_client: &Client,
    repo_info: &RepoInfo,
    issue_number: &i64,
    body: &str,
) -> Result<String, Box<dyn Error>> {
    let request = PullsUpdateReviewRequest {
        body: body.to_owned(),
    };

    let comment = github_client
        .issues()
        .create_comment(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            *issue_number,
            &request,
        )
        .await;

    match comment {
        Ok(_) => Ok("Comment create successed".to_string()),
        Err(er) => Err(Box::new(er)),
    }
}

// Get all Comments for issue/pull request without review comments
pub async fn get_all(
    github_client: &Client,
    repo_info: &RepoInfo,
    number: &i64,
) -> Result<Vec<IssueComment>, Box<dyn Error>> {
    let list_comments = github_client
        .issues()
        .list_all_comments(&repo_info.get_owner(), &repo_info.get_name(), *number, None)
        .await;

    match list_comments {
        Ok(c) => Ok(c.body),
        Err(er) => Err(Box::new(er)),
    }
}

// Get all review Comments for pull request
pub async fn get_all_from_review(
    github_client: &Client,
    repo_info: &RepoInfo,
    number: &i64,
    sort: Sort,
    direction: Order,
) -> Result<Vec<PullRequestReviewComment>, Box<dyn Error>> {
    let list_comments = github_client
        .pulls()
        .list_all_review_comments(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            *number,
            sort,
            direction,
            None,
        )
        .await;

    match list_comments {
        Ok(c) => Ok(c.body),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn update(
    github_client: &Client,
    repo_info: &RepoInfo,
    comment_id: &i64,
    body: &str,
) -> Result<String, Box<dyn Error>> {
    let request = PullsUpdateReviewRequest {
        body: body.to_owned(),
    };

    let comment = github_client
        .issues()
        .update_comment(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            *comment_id,
            &request,
        )
        .await;

    match comment {
        Ok(_) => Ok("Comment update successed".to_string()),
        Err(er) => Err(Box::new(er)),
    }
}
