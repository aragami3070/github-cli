use std::process;

use octorust::{types::ReposCreateReleaseRequest, Client};

pub async fn create(
    github_client: &Client,
    owner: &String,
    repo: &String,
    body: String,
    discussion_category_name: String,
    draft: Option<bool>,
    name: &String,
    prerelease: Option<bool>,
    tag_name: &String,
    target_commitish: String,
) -> String {

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
        .create_release(owner.trim(), repo.trim(), &request)
        .await;

    return match result {
        Ok(_) => "Success".to_string(),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    };
}
