use std::process;

use octorust::{types::ReposCreateReleaseRequest, Client};

use crate::git_utils::repo_info::RepoInfo;

pub async fn create(
    github_client: &Client,
    repo_info: RepoInfo,
    body: String,
    discussion_category_name: String,
    draft: Option<bool>,
    name: &String,
    prerelease: Option<bool>,
    tag_name: &String,
    target_commitish: String,
) -> (String, String) {
    let request = ReposCreateReleaseRequest {
        body: body,
        discussion_category_name: discussion_category_name,
        draft: draft,
        name: name.clone(),
        prerelease: prerelease,
        tag_name: tag_name.clone(),
        target_commitish: target_commitish,
    };

    let result = github_client
        .repos()
        .create_release(
            repo_info.get_owner().trim(),
            repo_info.get_name().trim(),
            &request,
        )
        .await;

    return match result {
        Ok(_) => ("Success".to_string(), repo_info.get_release_url(tag_name)),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    };
}
