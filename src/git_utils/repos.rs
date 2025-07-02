use std::process;

use octorust::{
    types::{ReposCreateRequest},
    Client,
};

pub async fn create_repo_for_authenticated_user(
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
