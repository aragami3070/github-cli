use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Work with issues
    Issue {
        #[command(subcommand)]
        subcommand: IssueCommand,
    },

    /// Work with comment
    Comment {
        #[command(subcommand)]
        subcommand: CommentCommand,
    },

    /// Work with repos
	Repo {
        #[command(subcommand)]
        subcommand: RepoCommand,
	}
}

#[derive(Subcommand)]
pub enum IssueCommand {
    /// Get list of issues
    List {
        /// The user that created the issues (optional)
        #[clap(long, short, default_value = "")]
        creator: String,
        /// Can be the name of a user. Pass in `none` for issues with no assigned user, and `*` for issues assigned to any user (optional)
        #[clap(long, short, default_value = "none")]
        assignee: String,
        /// Indicates the state of the issues to return. Can be either `open`, `closed`, or `all` (optional)
        #[clap(long, short, default_value = "open")]
        state: String,
        /// A list of comma separated label names. Example: `bug,ui,@high` (optional)
        #[clap(long, short, default_value = "")]
        labels: String,
        /// Page number of the results to fetch (optional)
        #[clap(long, short, default_value = "1")]
        numb_of_page: i64,
        /// Results on page (max 100) (optional)
        #[clap(long, short, default_value = "30", value_parser = clap::value_parser!(i64).range(1..=100))]
        iss_on_page: i64,
    },

    /// Create issue
    Create {
        /// Issue title
        #[clap(long, short)]
        title: String,
        /// Issue body (optional)
        #[clap(long, short, default_value = "")]
        body: String,
        /// A list of comma separated assignee names. Example: `aragami3070,danilasar` (optional)
        #[clap(long, short, default_value = "")]
        assignees: String,
        /// A list of comma separated label names. Example: `bug,ui,@high` (optional)
        #[clap(long, short, default_value = "enhancement")]
        labels: String,
    },

    /// Update issue
    Update {
        /// Update issue with number
        #[clap(long, short)]
        number: i64,
        /// Issue title
        #[clap(long, short, default_value = "None")]
        title: String,
        /// Issue body (optional)
        #[clap(long, short, default_value = "None")]
        body: String,
        /// A list of comma separated assignee names. Example: `aragami3070,danilasar` (optional)
        #[clap(long, short, default_value = None)]
        assignees: Option<String>,
        /// Indicates the state of the issues to return. Can be either `open` or `closed`(optional)
        #[clap(long, short, default_value = "open")]
        state: String,
        /// A list of comma separated label names. Example: `bug,ui,@high` (optional)
        #[clap(long, short, default_value = None)]
        labels: Option<String>,
    },

    /// Close issue
    Close {
        /// Close issue with number
        #[clap(long, short)]
        number: i64,
        /// Close with comment (optional)
        #[clap(long, short, default_value = "")]
        comment: String,
    },
}

#[derive(Subcommand)]
pub enum CommentCommand {
    /// Create new comment for issue/pull request
    Create {
        /// Create comment for issue/pull request with number
        #[clap(long, short)]
        number: i64,
        /// Comment body (optional)
        #[clap(long, short, default_value = "")]
        body: String,
    },
}

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
