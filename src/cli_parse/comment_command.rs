use clap::{Subcommand};

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
