use std::process;

use octorust::types::PullsUpdateReviewRequest;
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
