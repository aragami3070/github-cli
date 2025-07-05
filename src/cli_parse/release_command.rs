use clap::{Subcommand};

use crate::git_utils::repo_info::{RepoName, RepoOwner};

#[derive(Subcommand)]
pub enum ReleaseCommand {
    /// Create new release
    Create {
        #[clap(long, short)]
        owner: RepoOwner,
        #[clap(long, short)]
        repo: RepoName,
        #[clap(long, short, default_value = "")]
        body: String,
        #[clap(long, short)]
        name: String,
        #[clap(long, default_value = "")]
        discussion_category_name: String,
        #[clap(long, default_value = None)]
        draft: Option<bool>,
        #[clap(long, default_value = None)]
        prerelease: Option<bool>,
        #[clap(long)]
        tag_name: String,
        #[clap(long)]
        target_commitish: String,
    },
}
