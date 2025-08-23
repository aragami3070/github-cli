use octorust::{self, Client};
use std::error::Error;

use crate::cli_in::release_command::ReleaseCommand;
use crate::cli_out::print_in_cli::print_release;
use crate::cli_out::print_in_cli::print_url;
use crate::cli_parse::entities::CreateReleaseArgs;
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
            let command_args = CreateReleaseArgs {
                body,
                name,
                tag_name,
                target_commitish,
                discussion_category_name,
                draft,
                prerelease,
            };
            handle_create(github_client, owner, repo, command_args).await?;
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
    command_args: CreateReleaseArgs,
) -> Result<(), Box<dyn Error>> {
    let repo_info: RepoInfo = RepoInfo::new(Repo::Input, Some(owner), Some(repo))?;

    let result = releases::create(&github_client, repo_info, command_args).await?;

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
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    tag: String,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };

    let result = releases::get_by_tag(&github_client, repo_info, tag).await?;

    print_release(result);
    Ok(())
}

async fn handle_get_by_id(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    id: i64,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };

    let result = releases::get_by_id(&github_client, repo_info, id).await?;

    print_release(result);
    Ok(())
}
