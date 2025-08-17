use clap::Subcommand;

use crate::git_utils::repo_info::{RepoName, RepoOwner};

#[derive(Subcommand)]
pub enum ReleaseCommand {
    /// Create new release
    Create {
        /// Repo owner
        #[clap(long, short)]
        owner: RepoOwner,
        /// Repo name
        #[clap(long, short)]
        repo: RepoName,
        /// Tag name
        #[clap(long)]
        tag_name: String,
        /// Target commit hash (only long variant of commit hash) (optional)
        #[clap(long, default_value = "")]
        target_commitish: String,
        /// Release name
        #[clap(long, short)]
        name: String,
        /// Release body (optional)
        #[clap(long, short, default_value = "")]
        body: String,
        /// Name of discussion category (optional)
        #[clap(long, default_value = "")]
        discussion_category_name: String,
        /// It's draft? (optional)
        #[clap(long, default_value = None)]
        draft: Option<bool>,
        /// It's prerelease? (optional)
        #[clap(long, default_value = None)]
        prerelease: Option<bool>,
    },

    /// Get latest release
    GetLatest {
        /// Repo owner
        #[clap(long, short)]
        owner: RepoOwner,
        /// Repo name
        #[clap(long, short)]
        repo: RepoName,
    },

    /// Get release by tag
    GetByTag {
        /// Repo owner
        #[clap(long, short)]
        owner: RepoOwner,
        /// Repo name
        #[clap(long, short)]
        repo: RepoName,
        /// Release tag
        #[clap(long, short)]
        tag: String,
    },

    /// Get release by id
    GetById {
        /// Repo owner
        #[clap(long, short)]
        owner: RepoOwner,
        /// Repo name
        #[clap(long, short)]
        repo: RepoName,
        /// Release id
        #[clap(long, short)]
        id: i64,
    },
}
