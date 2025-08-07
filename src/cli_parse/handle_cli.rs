use std::error::Error;

use octorust::{self, Client};

use crate::cli_in::read_cli::Args;
use crate::cli_in::read_cli::CliCommand;
use crate::cli_parse::handle_commands::handle_comment::handle_comment_command;
use crate::cli_parse::handle_commands::handle_issue::handle_issue_command;
use crate::cli_parse::handle_commands::handle_release::handle_release_command;
use crate::cli_parse::handle_commands::handle_repo::handle_repo_command;

pub async fn handle_cli_command(args: Args, github_client: Client) -> Result<(), Box<dyn Error>> {
    match args.command {
        CliCommand::Issue { subcommand } => {
            handle_issue_command(github_client, subcommand).await?;
			Ok(())
        }

        CliCommand::Comment { subcommand } => {
            handle_comment_command(github_client, subcommand).await?;
			Ok(())
        }

        CliCommand::Repo { subcommand } => {
            handle_repo_command(github_client, subcommand).await;
			Ok(())
        }

        CliCommand::Release { subcommand } => {
            handle_release_command(github_client, subcommand).await;
			Ok(())
        }
    }
}
