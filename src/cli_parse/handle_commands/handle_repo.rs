use octorust::{self, Client};
use std::error::Error;

use crate::cli_in::repo_command::RepoCommand;
use crate::cli_in::set_vars::Orders;
use crate::cli_in::set_vars::ReposListOrgSorts;
use crate::cli_in::set_vars::ReposListOrgTypes;
use crate::cli_in::set_vars::ReposListUserTypes;
use crate::cli_in::set_vars::Visibilities;
use crate::cli_out::print_in_cli::print_repos;
use crate::cli_out::print_in_cli::print_url;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;
use crate::git_utils::repo_info::{RepoName, RepoOwner};
use crate::git_utils::repos;
use crate::cli_parse::entities::CreateRepoArgs;

pub async fn handle_repo_command(
    github_client: Client,
    subcommand: RepoCommand,
) -> Result<(), Box<dyn Error>> {
    match subcommand {
        RepoCommand::CreateForAuthenticatedUser {
            allow_auto_merge,
            allow_merge_commit,
            allow_rebase_merge,
            allow_squash_merge,
            auto_init,
            delete_branch_on_merge,
            description,
            gitignore_template,
            has_issues,
            has_projects,
            has_wiki,
            homepage,
            is_template,
            license_template,
            name,
            private,
        } => {
            let command_args = CreateRepoArgs {
                allow_auto_merge,
                allow_merge_commit,
                allow_rebase_merge,
                allow_squash_merge,
                auto_init,
                delete_branch_on_merge,
                has_issues,
                has_projects,
                has_wiki,
                is_template,
                private,
                description,
                gitignore_template,
                homepage,
                license_template,
                name,
            };

            handle_create_for_auth_user(
                github_client,
				command_args
            )
            .await?;
            Ok(())
        }

        RepoCommand::CreateInOrg {
            allow_auto_merge,
            allow_merge_commit,
            allow_rebase_merge,
            allow_squash_merge,
            auto_init,
            delete_branch_on_merge,
            description,
            gitignore_template,
            has_issues,
            has_projects,
            has_wiki,
            homepage,
            is_template,
            license_template,
            name,
            org,
            team_name,
            visibility,
        } => {
            handle_create_in_org(
                github_client,
                allow_auto_merge,
                allow_merge_commit,
                allow_rebase_merge,
                allow_squash_merge,
                auto_init,
                delete_branch_on_merge,
                description,
                gitignore_template,
                has_issues,
                has_projects,
                has_wiki,
                homepage,
                is_template,
                license_template,
                name,
                org,
                team_name,
                visibility,
            )
            .await?;
            Ok(())
        }

        RepoCommand::GetAllFromOrg {
            org,
            order,
            sort_value,
            type_value,
        } => {
            handle_get_all_from_org(github_client, org, order, type_value, sort_value).await?;
            Ok(())
        }

        RepoCommand::CreateUsingTemplate {
            template_owner,
            template_name,
            name,
            owner,
            description,
            private,
            include_all_branches,
        } => {
            handle_create_using_template(
                github_client,
                owner,
                name,
                template_owner,
                template_name,
                description,
                include_all_branches,
                private,
            )
            .await?;
            Ok(())
        }

        RepoCommand::CreateFork { org, name, owner } => {
            handle_create_fork(github_client, owner, name, org).await?;
            Ok(())
        }

        RepoCommand::GetAllFromUser {
            owner,
            order,
            sort_value,
            type_value,
        } => {
            handle_get_all_from_user(github_client, owner, type_value, sort_value, order).await?;
            Ok(())
        }
    }
}

async fn handle_create_for_auth_user(
    github_client: Client,
	command_args: CreateRepoArgs,
) -> Result<(), Box<dyn Error>> {
    let result = repos::create_for_authenticated_user(
        &github_client,
		command_args,
    )
    .await?;

    println!("{result}");
    Ok(())
}

async fn handle_create_in_org(
    github_client: Client,
    allow_auto_merge: Option<bool>,
    allow_merge_commit: Option<bool>,
    allow_rebase_merge: Option<bool>,
    allow_squash_merge: Option<bool>,
    auto_init: Option<bool>,
    delete_branch_on_merge: Option<bool>,
    description: String,
    gitignore_template: String,
    has_issues: Option<bool>,
    has_projects: Option<bool>,
    has_wiki: Option<bool>,
    homepage: String,
    is_template: Option<bool>,
    license_template: String,
    name: RepoName,
    org: RepoOwner,
    team_name: String,
    visibility: Visibilities,
) -> Result<(), Box<dyn Error>> {
    let repo_info: RepoInfo = RepoInfo::new(Repo::Input, Some(org), Some(name))?;

    let result = repos::create_in_org(
        &github_client,
        repo_info,
        allow_auto_merge,
        allow_merge_commit,
        allow_rebase_merge,
        allow_squash_merge,
        auto_init,
        delete_branch_on_merge,
        &description,
        &gitignore_template,
        has_issues,
        has_projects,
        has_wiki,
        &homepage,
        is_template,
        &license_template,
        &team_name,
        Some(visibility.0),
    )
    .await?;

    print_url(result, "New repo");
    Ok(())
}

async fn handle_get_all_from_org(
    github_client: Client,
    org: String,
    order: Orders,
    type_value: ReposListOrgTypes,
    sort_value: ReposListOrgSorts,
) -> Result<(), Box<dyn Error>> {
    let all_repos =
        repos::get_all_from_org(&github_client, &org, order.0, type_value.0, sort_value.0).await?;

    print_repos(all_repos, org, "org");
    Ok(())
}

async fn handle_create_using_template(
    github_client: Client,
    owner: RepoOwner,
    name: RepoName,
    template_owner: RepoOwner,
    template_name: RepoName,
    description: String,
    include_all_branches: Option<bool>,
    private: Option<bool>,
) -> Result<(), Box<dyn Error>> {
    let repo_info: RepoInfo = RepoInfo::new(Repo::Input, Some(owner), Some(name))?;
    let template_info: RepoInfo =
        RepoInfo::new(Repo::Input, Some(template_owner), Some(template_name))?;

    let result = repos::create_using_template(
        &github_client,
        template_info,
        repo_info,
        &description,
        include_all_branches,
        private,
    )
    .await?;

    print_url(result, "New repo");
    Ok(())
}

async fn handle_create_fork(
    github_client: Client,
    owner: RepoOwner,
    name: RepoName,
    org: String,
) -> Result<(), Box<dyn Error>> {
    let fork_info: RepoInfo = RepoInfo::new(Repo::Input, Some(owner), Some(name))?;
    let result = repos::create_fork(&github_client, &org, fork_info).await?;

    println!("{result}");
    Ok(())
}

async fn handle_get_all_from_user(
    github_client: Client,
    owner: String,
    type_value: ReposListUserTypes,
    sort_value: ReposListOrgSorts,
    order: Orders,
) -> Result<(), Box<dyn Error>> {
    let result = repos::get_all_from_user(
        &github_client,
        owner.clone(),
        type_value.0,
        sort_value.0,
        order.0,
    )
    .await?;

    print_repos(result, owner, "user");
    Ok(())
}
