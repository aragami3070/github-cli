use crate::cli_parse::comment_command::CommentCommand;
use crate::cli_parse::issue_command::IssueCommand;
use crate::cli_parse::release_command::ReleaseCommand;
use crate::cli_parse::repo_command::RepoCommand;
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
