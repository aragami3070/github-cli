use clap::{Subcommand};

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
        name: String,
        #[clap(long)]
        org: String,
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
        template_owner: String,
		/// Template repo name
        #[clap(long)]
        template_name: String,
		/// New repo name
        #[clap(long, short)]
        name: String,
		/// New repo owner name
        #[clap(long, short)]
        owner: String,
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
        name: String,
		/// Template owner name
        #[clap(long, short)]
        owner: String,
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
        order: String,
        /// Sort can be only 'created', 'fullname', 'pushed', or 'updated' (optional)
        #[clap(long, short, default_value = "updated")]
        sort_value: String,
		/// Type can be 'all', 'forks', 'internal', 'member', 'private', 'public' or 'sources' (optional)
        #[clap(long, short, default_value = "all")]
        type_value: String,
    },
}
