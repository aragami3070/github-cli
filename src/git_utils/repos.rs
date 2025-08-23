use std::error::Error;

use octorust::{
    types::{
        MinimalRepository, Order, ReposCreateForkRequest, ReposCreateInOrgRequest,
        ReposCreateInOrgRequestVisibility, ReposCreateRequest, ReposCreateUsingTemplateRequest,
        ReposListOrgSort, ReposListOrgType, ReposListUserType,
    },
    Client,
};

use crate::cli_parse::entities::{CreateRepoArgs, CreateRepoFromTemplateArgs};
use crate::git_utils::{repo_info::RepoInfo, teams::get_id};

pub async fn create_for_authenticated_user(
    github_client: &Client,
    command_args: CreateRepoArgs,
) -> Result<String, Box<dyn Error>> {
    let request = ReposCreateRequest {
        allow_auto_merge: command_args.allow_auto_merge,
        allow_merge_commit: command_args.allow_merge_commit,
        allow_rebase_merge: command_args.allow_rebase_merge,
        allow_squash_merge: command_args.allow_squash_merge,
        auto_init: command_args.auto_init,
        delete_branch_on_merge: command_args.delete_branch_on_merge,
        description: command_args.description.clone(),
        gitignore_template: command_args.gitignore_template.clone(),
        has_downloads: None,
        has_issues: command_args.has_issues,
        has_projects: command_args.has_projects,
        has_wiki: command_args.has_wiki,
        homepage: command_args.homepage.clone(),
        is_template: command_args.is_template,
        license_template: command_args.license_template.clone(),
        name: command_args.name.clone(),
        private: command_args.private,
        team_id: 1,
    };

    let new_repo = github_client
        .repos()
        .create_for_authenticated_user(&request)
        .await;

    match new_repo {
        Ok(_) => Ok("Success".to_string()),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn create_in_org(
    github_client: &Client,
    repo_info: RepoInfo,
    command_args: CreateRepoArgs,
    team_name: &str,
    visibility: Option<ReposCreateInOrgRequestVisibility>,
) -> Result<String, Box<dyn Error>> {
    let team = get_id(github_client, &repo_info.get_owner(), team_name).await?;

    let team_id = team.id;

    let request = ReposCreateInOrgRequest {
        allow_auto_merge: command_args.allow_auto_merge,
        allow_merge_commit: command_args.allow_merge_commit,
        allow_rebase_merge: command_args.allow_rebase_merge,
        allow_squash_merge: command_args.allow_squash_merge,
        auto_init: command_args.auto_init,
        delete_branch_on_merge: command_args.delete_branch_on_merge,
        description: command_args.description.clone(),
        gitignore_template: command_args.gitignore_template.clone(),
        has_issues: command_args.has_issues,
        has_projects: command_args.has_projects,
        has_wiki: command_args.has_wiki,
        homepage: command_args.homepage.clone(),
        is_template: command_args.is_template,
        license_template: command_args.license_template.clone(),
        name: repo_info.get_name(),
        private: None,
        team_id,
        visibility,
    };

    let new_repo = github_client
        .repos()
        .create_in_org(repo_info.get_owner().trim(), &request)
        .await;

    match new_repo {
        Ok(_) => Ok(repo_info.get_ssh()),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn get_all_from_org(
    github_client: &Client,
    org: &str,
    order: Order,
    type_value: ReposListOrgType,
    sort_value: ReposListOrgSort,
) -> Result<Vec<MinimalRepository>, Box<dyn Error>> {
    let all_repos = github_client
        .repos()
        .list_all_for_org(org.trim(), type_value, sort_value, order)
        .await;

    match all_repos {
        Ok(r) => Ok(r.body),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn create_using_template(
    github_client: &Client,
    template_info: RepoInfo,
    repo_info: RepoInfo,
    command_args: CreateRepoFromTemplateArgs,
) -> Result<String, Box<dyn Error>> {
    let request = ReposCreateUsingTemplateRequest {
        description: command_args.description,
        include_all_branches: command_args.include_all_branches,
        name: repo_info.get_name(),
        owner: repo_info.get_owner(),
        private: command_args.private,
    };

    let new_repo = github_client
        .repos()
        .create_using_template(
            &template_info.get_owner(),
            &template_info.get_name(),
            &request,
        )
        .await;

    match new_repo {
        Ok(_) => Ok(repo_info.get_ssh()),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn create_fork(
    github_client: &Client,
    org: &str,
    fork_info: RepoInfo,
) -> Result<String, Box<dyn Error>> {
    let request = ReposCreateForkRequest {
        organization: org.to_owned(),
    };

    let new_fork = github_client
        .repos()
        .create_fork(&fork_info.get_owner(), &fork_info.get_name(), &request)
        .await;

    match new_fork {
        Ok(_) => Ok("Success".to_string()),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn get_all_from_user(
    github_client: &Client,
    owner: String,
    type_value: ReposListUserType,
    sort_value: ReposListOrgSort,
    order: Order,
) -> Result<Vec<MinimalRepository>, Box<dyn Error>> {
    let all_repos = github_client
        .repos()
        .list_all_for_user(owner.trim(), type_value, sort_value, order)
        .await;

    match all_repos {
        Ok(reps) => Ok(reps.body),
        Err(er) => Err(Box::new(er)),
    }
}
