use crate::git_utils::repo_info::{RepoName, RepoOwner};
use clap::Subcommand;

use crate::cli_in::set_vars::{IssuesListStates, States};

#[derive(Subcommand)]
pub enum IssueCommand {
    /// Get list of issues
    List {
        /// Repo owner
        #[clap(long, short, default_value = None)]
        owner: Option<RepoOwner>,
        /// Repo name
        #[clap(long, short, default_value = None)]
        repo: Option<RepoName>,
        /// The user that created the issues (optional)
        #[clap(long, short, default_value = "")]
        creator: String,
        /// Can be the name of a user. Pass in `none` for issues with no assigned user, and `*` for issues assigned to any user (optional)
        #[clap(long, short, default_value = "none")]
        assignee: String,
        /// Indicates the state of the issues to return. Can be either `open`, `closed`, or `all` (optional)
        #[clap(long, short, default_value = "open")]
        state: IssuesListStates,
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
        /// Repo owner
        #[clap(long, short, default_value = None)]
        owner: Option<RepoOwner>,
        /// Repo name
        #[clap(long, short, default_value = None)]
        repo: Option<RepoName>,
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
        state: States,
        /// A list of comma separated label names. Example: `bug,ui,@high` (optional)
        #[clap(long, short, default_value = None)]
        labels: Option<String>,
    },

    /// Close issue
    Close {
        /// Repo owner
        #[clap(long, short, default_value = None)]
        owner: Option<RepoOwner>,
        /// Repo name
        #[clap(long, short, default_value = None)]
        repo: Option<RepoName>,
        /// Close issue with number
        #[clap(long, short)]
        number: i64,
        /// Close with comment (optional)
        #[clap(long, short, default_value = "")]
        comment: String,
    },
}
