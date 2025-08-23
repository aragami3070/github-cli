use std::error::Error;

use octorust::{
    types::{Release, ReposCreateReleaseRequest},
    Client,
};

use crate::git_utils::repo_info::RepoInfo;

pub async fn create(
    github_client: &Client,
    repo_info: RepoInfo,
    body: String,
    discussion_category_name: String,
    draft: Option<bool>,
    name: &str,
    prerelease: Option<bool>,
    tag_name: &str,
    target_commitish: String,
) -> Result<String, Box<dyn Error>> {
    let request = ReposCreateReleaseRequest {
        body,
        discussion_category_name,
        draft,
        name: name.to_owned(),
        prerelease,
        tag_name: tag_name.to_owned(),
        target_commitish,
    };

    let result = github_client
        .repos()
        .create_release(&repo_info.get_owner(), &repo_info.get_name(), &request)
        .await;

    match result {
        Ok(_) => Ok(repo_info.get_release_url(tag_name)),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn get_latest(
    github_client: &Client,
    repo_info: RepoInfo,
) -> Result<Release, Box<dyn Error>> {
    let result = github_client
        .repos()
        .get_latest_release(&repo_info.get_owner(), &repo_info.get_name())
        .await;

    match result {
        Ok(r) => Ok(r.body),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn get_by_tag(
    github_client: &Client,
    repo_info: RepoInfo,
    tag: String,
) -> Result<Release, Box<dyn Error>> {
    let result = github_client
        .repos()
        .get_release_by_tag(&repo_info.get_owner(), &repo_info.get_name(), &tag)
        .await;

    match result {
        Ok(r) => Ok(r.body),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn get_by_id(
    github_client: &Client,
    repo_info: RepoInfo,
    id: i64,
) -> Result<Release, Box<dyn Error>> {
    let result = github_client
        .repos()
        .get_release(&repo_info.get_owner(), &repo_info.get_name(), id)
        .await;

    match result {
        Ok(r) => Ok(r.body),
        Err(er) => Err(Box::new(er)),
    }
}
