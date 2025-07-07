use std::process;

use octorust::{
    types::{
        MinimalRepository, Order, ReposCreateForkRequest, ReposCreateInOrgRequest,
        ReposCreateInOrgRequestVisibility, ReposCreateRequest, ReposCreateUsingTemplateRequest,
        ReposListOrgSort, ReposListOrgType, ReposListType, ReposListUserType, ReposListVisibility,
        Repository,
    },
    Client,
};

use crate::git_utils::{repo_info::RepoInfo, teams::get_id};

pub async fn create_for_authenticated_user(
    github_client: &Client,
    allow_auto_merge: Option<bool>,
    allow_merge_commit: Option<bool>,
    allow_rebase_merge: Option<bool>,
    allow_squash_merge: Option<bool>,
    auto_init: Option<bool>,
    delete_branch_on_merge: Option<bool>,
    description: &String,
    gitignore_template: &String,
    has_issues: Option<bool>,
    has_projects: Option<bool>,
    has_wiki: Option<bool>,
    homepage: &String,
    is_template: Option<bool>,
    license_template: &String,
    name: &String,
    private: Option<bool>,
) -> String {
    let request = ReposCreateRequest {
        allow_auto_merge: allow_auto_merge,
        allow_merge_commit: allow_merge_commit,
        allow_rebase_merge: allow_rebase_merge,
        allow_squash_merge: allow_squash_merge,
        auto_init: auto_init,
        delete_branch_on_merge: delete_branch_on_merge,
        description: description.clone(),
        gitignore_template: gitignore_template.clone(),
        has_downloads: None,
        has_issues: has_issues,
        has_projects: has_projects,
        has_wiki: has_wiki,
        homepage: homepage.clone(),
        is_template: is_template,
        license_template: license_template.clone(),
        name: name.clone(),
        private: private,
        team_id: 1,
    };

    let new_repo = github_client
        .repos()
        .create_for_authenticated_user(&request)
        .await;

    return match new_repo {
        Ok(_) => "Success".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

pub async fn create_in_org(
    github_client: &Client,
    repo_info: RepoInfo,
    allow_auto_merge: Option<bool>,
    allow_merge_commit: Option<bool>,
    allow_rebase_merge: Option<bool>,
    allow_squash_merge: Option<bool>,
    auto_init: Option<bool>,
    delete_branch_on_merge: Option<bool>,
    description: &String,
    gitignore_template: &String,
    has_issues: Option<bool>,
    has_projects: Option<bool>,
    has_wiki: Option<bool>,
    homepage: &String,
    is_template: Option<bool>,
    license_template: &String,
    team_name: &String,
    visibility: Option<ReposCreateInOrgRequestVisibility>,
) -> String {
    let team = get_id(github_client, &repo_info.get_owner(), team_name).await;

    let team_id = team.id;

    let request = ReposCreateInOrgRequest {
        allow_auto_merge: allow_auto_merge,
        allow_merge_commit: allow_merge_commit,
        allow_rebase_merge: allow_rebase_merge,
        allow_squash_merge: allow_squash_merge,
        auto_init: auto_init,
        delete_branch_on_merge: delete_branch_on_merge,
        description: description.clone(),
        gitignore_template: gitignore_template.clone(),
        has_issues: has_issues,
        has_projects: has_projects,
        has_wiki: has_wiki,
        homepage: homepage.clone(),
        is_template: is_template,
        license_template: license_template.clone(),
        name: repo_info.get_name(),
        private: None,
        team_id: team_id,
        visibility: visibility,
    };

    let new_repo = github_client
        .repos()
        .create_in_org(repo_info.get_owner().trim(), &request)
        .await;

    return match new_repo {
        Ok(_) => repo_info.get_ssh(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

pub async fn get_all_from_org(
    github_client: &Client,
    org: &String,
    order: Order,
    type_value: ReposListOrgType,
    sort_value: ReposListOrgSort,
) -> Vec<MinimalRepository> {
    let all_repos = github_client
        .repos()
        .list_all_for_org(org.trim(), type_value, sort_value, order)
        .await;

    return match all_repos {
        Ok(r) => r.body,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

pub async fn create_using_template(
    github_client: &Client,
    template_info: RepoInfo,
    repo_info: RepoInfo,
    description: &String,
    include_all_branches: Option<bool>,
    private: Option<bool>,
) -> String {
    let request = ReposCreateUsingTemplateRequest {
        description: description.clone(),
        include_all_branches: include_all_branches,
        name: repo_info.get_name(),
        owner: repo_info.get_owner(),
        private: private,
    };

    let new_repo = github_client
        .repos()
        .create_using_template(
            &template_info.get_owner().trim(),
            &template_info.get_name().trim(),
            &request,
        )
        .await;

    return match new_repo {
        Ok(_) => repo_info.get_ssh(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

pub async fn create_fork(github_client: &Client, org: &String, fork_info: RepoInfo) -> String {
    let request = ReposCreateForkRequest {
        organization: org.clone(),
    };

    let new_fork = github_client
        .repos()
        .create_fork(
            &fork_info.get_owner().trim(),
            &fork_info.get_name().trim(),
            &request,
        )
        .await;

    return match new_fork {
        Ok(_) => "Success".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

pub async fn get_all_from_user(
    github_client: &Client,
    owner: String,
    type_value: ReposListUserType,
    sort_value: ReposListOrgSort,
    order: Order,
) -> Vec<MinimalRepository> {
    let all_repos = github_client
        .repos()
        .list_all_for_user(owner.trim(), type_value, sort_value, order)
        .await;

    return match all_repos {
        Ok(reps) => reps.body,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
