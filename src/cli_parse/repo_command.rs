use clap::Subcommand;

use crate::{cli_parse::set_vars::Orders, git_utils::repo_info::{RepoName, RepoOwner}};

#[derive(Subcommand)]
pub enum RepoCommand {
    /// Create new repo in your ownership
    CreateForAuthenticatedUser {
        #[clap(long, default_value = None)]
        allow_auto_merge: Option<bool>,
        #[clap(long, default_value = None)]
        allow_merge_commit: Option<bool>,
        #[clap(long, default_value = None)]
        allow_rebase_merge: Option<bool>,
        #[clap(long, default_value = None)]
        allow_squash_merge: Option<bool>,
        #[clap(long, default_value = None)]
        auto_init: Option<bool>,
        #[clap(long, default_value = None)]
        delete_branch_on_merge: Option<bool>,
        #[clap(long, default_value = "")]
        description: String,
        #[clap(long, default_value = "")]
        gitignore_template: String,
        #[clap(long, default_value = None)]
        has_issues: Option<bool>,
        #[clap(long, default_value = None)]
        has_projects: Option<bool>,
        #[clap(long, default_value = None)]
        has_wiki: Option<bool>,
        #[clap(long, default_value = "")]
        homepage: String,
        #[clap(long, default_value = None)]
        is_template: Option<bool>,
        #[clap(long, default_value = "")]
        license_template: String,
        #[clap(long)]
        name: String,
        #[clap(long, default_value = None)]
        private: Option<bool>,
    },

    /// Create new repo in org
    CreateInOrg {
        #[clap(long, default_value = None)]
        allow_auto_merge: Option<bool>,
        #[clap(long, default_value = None)]
        allow_merge_commit: Option<bool>,
        #[clap(long, default_value = None)]
        allow_rebase_merge: Option<bool>,
        #[clap(long, default_value = None)]
        allow_squash_merge: Option<bool>,
        #[clap(long, default_value = None)]
        auto_init: Option<bool>,
        #[clap(long, default_value = None)]
        delete_branch_on_merge: Option<bool>,
        #[clap(long, default_value = "")]
        description: String,
        #[clap(long, default_value = "")]
        gitignore_template: String,
        #[clap(long, default_value = None)]
        has_issues: Option<bool>,
        #[clap(long, default_value = None)]
        has_projects: Option<bool>,
        #[clap(long, default_value = None)]
        has_wiki: Option<bool>,
        #[clap(long, default_value = "")]
        homepage: String,
        #[clap(long, default_value = None)]
        is_template: Option<bool>,
        #[clap(long, default_value = "")]
        license_template: String,
        #[clap(long)]
        name: RepoName,
        #[clap(long)]
        org: RepoOwner,
        #[clap(long)]
        team_name: String,
        /// Can be 'public', 'private' and 'internal' for Enterprises
        #[clap(long, default_value = "public")]
        visibility: String,
    },

    /// Create new repo usnig template
    CreateUsingTemplate {
        /// Template owner name
        #[clap(long)]
        template_owner: RepoOwner,
        /// Template repo name
        #[clap(long)]
        template_name: RepoName,
        /// New repo name
        #[clap(long, short)]
        name: RepoName,
        /// New repo owner name
        #[clap(long, short)]
        owner: RepoOwner,
        /// New repo description (optional)
        #[clap(long, short, default_value = "")]
        description: String,
        /// Repo visiblity (optional)
        #[clap(long, short, default_value = None)]
        private: Option<bool>,
        /// Include all branches from template (optional)
        #[clap(long, short, default_value = None)]
        include_all_branches: Option<bool>,
    },

    /// Create fork
    CreateFork {
        /// Template name
        #[clap(long, short)]
        name: RepoName,
        /// Template owner name
        #[clap(long, short)]
        owner: RepoOwner,
        /// Org name if forking into an org. Else forking in account token owner (optional)
        #[clap(long, default_value = "")]
        org: String,
    },

    /// Get all repos from org
    GetAllFromOrg {
        /// Org name
        #[clap(long)]
        org: String,
        /// Order can be only 'asc' or 'desc' (optional)
        #[clap(long, short, default_value = "desc")]
        order: Orders,
        /// Sort can be only 'created', 'fullname', 'pushed', or 'updated' (optional)
        #[clap(long, short, default_value = "updated")]
        sort_value: String,
        /// Type can be 'all', 'forks', 'internal', 'member', 'private', 'public' or 'sources' (optional)
        #[clap(long, short, default_value = "all")]
        type_value: String,
    },

    /// Get all repos from user
    GetAllFromUser {
        /// Owner name 
        #[clap(long)]
        owner: String,
        /// Order can be only 'asc' or 'desc' (optional)
        #[clap(long, short, default_value = "desc")]
        order: Orders,
        /// Sort can be only 'created', 'fullname', 'pushed', or 'updated' (optional)
        #[clap(long, short, default_value = "updated")]
        sort_value: String,
        /// Type can be 'all', 'member', or 'owner' always and can be 'private', 'public' if owner
        /// not 'me' (optional)
        #[clap(long, short, default_value = "all")]
        type_value: String,
        ///// NOT WORKING NOW BECAUSE OCTORUST BREAK THIS
		///// Can include:
        ///// - owner: Repositories that are owned by the authenticated user.
        ///// - collaborator: Repositories that the user has been added to as a collaborator.
        ///// - organization_member: Repositories that the user has access to through being a member of an organization. This includes every repository on every team that the user is on.
        // #[clap(long, short, verbatim_doc_comment, default_value = "")]
        // affiliation: String,
    },
}
