use clap::Subcommand;

use crate::cli_in::set_vars::CommentTarget;

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

    /// Get all comments from issue/pull request
    GetAll {
        /// Get all comments from issue/pull request with number
        #[clap(long, short)]
        number: i64,
        /// Get from issue or from pull request (can be only 'issue' or 'pull-request')
        #[clap(long, short)]
        target: CommentTarget,
    },
}
