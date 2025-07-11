use crate::cli_in::comment_command::CommentCommand;
use crate::cli_in::issue_command::IssueCommand;
use crate::cli_in::release_command::ReleaseCommand;
use crate::cli_in::repo_command::RepoCommand;
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

    /// Work with comments
    Comment {
        #[command(subcommand)]
        subcommand: CommentCommand,
    },

    /// Work with repos
    Repo {
        #[command(subcommand)]
        subcommand: RepoCommand,
    },

    /// Work with releases
    Release {
        #[command(subcommand)]
        subcommand: ReleaseCommand,
    },
}
