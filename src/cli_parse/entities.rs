use crate::cli_in::set_vars::{IssuesListStates, States};

pub struct ListIssueArgs {
    pub creator: String,
    pub assignee: String,
    pub state: IssuesListStates,
    pub labels: String,
    pub numb_of_page: i64,
    pub iss_on_page: i64,
}

pub struct UpdateIssueArgs {
    pub title: Option<String>,
    pub body: Option<String>,
    pub number: i64,
    pub state: States,
}

pub struct CreateRepoArgs {
    pub allow_auto_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub auto_init: Option<bool>,
    pub delete_branch_on_merge: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_wiki: Option<bool>,
    pub is_template: Option<bool>,
    pub private: Option<bool>,
    pub description: String,
    pub gitignore_template: String,
    pub homepage: String,
    pub license_template: String,
    pub name: String,
}

pub struct CreateRepoFromTemplateArgs {
	pub description: String,
	pub include_all_branches: Option<bool>,
	pub private: Option<bool>,
}
