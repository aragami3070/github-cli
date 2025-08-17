use crate::git_utils::repo_info::{RepoName, RepoOwner};
use clap::Subcommand;

use crate::cli_in::set_vars::{CommentTarget, Orders, Sorts};

#[derive(Subcommand)]
pub enum CommentCommand {
    /// Create new comment for issue/pull request
    Create {
        /// Repo owner
        #[clap(long, short, default_value = None)]
        owner: Option<RepoOwner>,
        /// Repo name
        #[clap(long, short, default_value = None)]
        repo: Option<RepoName>,
        /// Create comment for issue/pull request with number
        #[clap(long, short)]
        number: i64,
        /// Comment body (optional)
        #[clap(long, short, default_value = "")]
        body: String,
    },

    /// Get all comments from issue/pull request
    GetAll {
        /// Repo owner
        #[clap(long, short, default_value = None)]
        owner: Option<RepoOwner>,
        /// Repo name
        #[clap(long, short, default_value = None)]
        repo: Option<RepoName>,
        /// Get all comments from issue/pull request with number
        #[clap(long, short)]
        number: i64,
        /// Get from issue or from pull request (can be only 'issue' or 'pull-request')
        #[clap(long, short)]
        target: CommentTarget,
    },

    /// Get all comments from issue/pull request
    GetAllFromReview {
        /// Repo owner
        #[clap(long, short, default_value = None)]
        owner: Option<RepoOwner>,
        /// Repo name
        #[clap(long, short, default_value = None)]
        repo: Option<RepoName>,
        /// Get all comments from issue/pull request with number
        #[clap(long, short)]
        number: i64,
        /// Can be only 'created' or 'updated' (optional)
        #[clap(long, short, default_value = "created")]
        sort: Sorts,
        /// Can be only 'asc' or 'desc' (optional)
        #[clap(long, default_value = "desc")]
        order: Orders,
    },
}
