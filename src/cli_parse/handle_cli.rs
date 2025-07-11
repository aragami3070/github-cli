use octorust::{self, Client};
use std::process;

use crate::cli_in::read_cli::Args;
use crate::cli_in::read_cli::CliCommand;
use crate::cli_in::release_command::ReleaseCommand;
use crate::cli_out::print_in_cli::print_release;
use crate::cli_out::print_in_cli::print_url;
use crate::cli_parse::handle_commands::handle_comment::handle_comment_command;
use crate::cli_parse::handle_commands::handle_issue::handle_issue_command;
use crate::cli_parse::handle_commands::handle_repo::handle_repo_command;
use crate::git_utils::releases;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;

pub async fn handle_cli_command(args: Args, github_client: Client) {
    match args.command {
        CliCommand::Issue { subcommand } => {
            handle_issue_command(github_client, subcommand).await;
        }

        CliCommand::Comment { subcommand } => {
            handle_comment_command(github_client, subcommand).await;
        }

        CliCommand::Repo { subcommand } => {
			handle_repo_command(github_client, subcommand).await;
        },

        CliCommand::Release { subcommand } => match subcommand {
            ReleaseCommand::Create {
                owner,
                repo,
                body,
                name,
                discussion_category_name,
                draft,
                prerelease,
                tag_name,
                target_commitish,
            } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo))
                {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let result = releases::create(
                    &github_client,
                    repo_info,
                    body,
                    discussion_category_name,
                    draft,
                    &name,
                    prerelease,
                    &tag_name,
                    target_commitish,
                )
                .await;

                print_url(result, "New release");
            }

            ReleaseCommand::GetLatest { owner, repo } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo))
                {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let result = releases::get_latest(&github_client, repo_info).await;

                print_release(result);
            }

            ReleaseCommand::GetByTag { owner, repo, tag } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo))
                {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let result = releases::get_by_tag(&github_client, repo_info, tag).await;

                print_release(result);
            }

            ReleaseCommand::GetById { owner, repo, id } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo))
                {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let result = releases::get_by_id(&github_client, repo_info, id).await;

                print_release(result);
            }
        },
    }
}
