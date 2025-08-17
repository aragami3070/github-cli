use octorust::{self, Client};
use std::error::Error;

use crate::cli_in::release_command::ReleaseCommand;
use crate::cli_out::print_in_cli::print_release;
use crate::cli_out::print_in_cli::print_url;
use crate::git_utils::releases;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;
use crate::git_utils::repo_info::RepoName;
use crate::git_utils::repo_info::RepoOwner;

pub async fn handle_release_command(
    github_client: Client,
    subcommand: ReleaseCommand,
) -> Result<(), Box<dyn Error>> {
    match subcommand {
        ReleaseCommand::Create {
            owner,
            repo,
            body,
            name,
            discussion_category_name,
            draft,
            prerelease,
            tag_name,
            target_commitish,
        } => {
            handle_create(
                github_client,
                owner,
                repo,
                body,
                discussion_category_name,
                name,
                draft,
                prerelease,
                tag_name,
                target_commitish,
            )
            .await?;
            Ok(())
        }

        ReleaseCommand::GetLatest { owner, repo } => {
            handle_get_latest(github_client, owner, repo).await?;
            Ok(())
        }

        ReleaseCommand::GetByTag { owner, repo, tag } => {
            handle_get_by_tag(github_client, owner, repo, tag).await?;
            Ok(())
        }

        ReleaseCommand::GetById { owner, repo, id } => {
            handle_get_by_id(github_client, owner, repo, id).await?;
            Ok(())
        }
    }
}

async fn handle_create(
    github_client: Client,
    owner: RepoOwner,
    repo: RepoName,
    body: String,
    discussion_category_name: String,
    name: String,
    draft: Option<bool>,
    prerelease: Option<bool>,
    tag_name: String,
    target_commitish: String,
) -> Result<(), Box<dyn Error>> {
    let repo_info: RepoInfo = RepoInfo::new(Repo::Input, Some(owner), Some(repo))?;

    let result = releases::create(
        &github_client,
        repo_info,
        body,
        discussion_category_name,
        draft,
        &name,
        prerelease,
        &tag_name,
        target_commitish,
    )
    .await?;

    print_url(result, "New release");
    Ok(())
}

async fn handle_get_latest(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };

    let result = releases::get_latest(&github_client, repo_info).await?;

    print_release(result);
    Ok(())
}

async fn handle_get_by_tag(
    github_client: Client,
    owner: RepoOwner,
    repo: RepoName,
    tag: String,
) -> Result<(), Box<dyn Error>> {
    let repo_info: RepoInfo = RepoInfo::new(Repo::Input, Some(owner), Some(repo))?;

    let result = releases::get_by_tag(&github_client, repo_info, tag).await?;

    print_release(result);
    Ok(())
}

async fn handle_get_by_id(
    github_client: Client,
    owner: RepoOwner,
    repo: RepoName,
    id: i64,
) -> Result<(), Box<dyn Error>> {
    let repo_info: RepoInfo = RepoInfo::new(Repo::Input, Some(owner), Some(repo))?;

    let result = releases::get_by_id(&github_client, repo_info, id).await?;

    print_release(result);
    Ok(())
}
