use crate::cli_in::comment_command::CommentCommand;
use crate::cli_in::read_cli::Args;
use crate::cli_in::read_cli::CliCommand;
use crate::cli_in::release_command::ReleaseCommand;
use crate::cli_in::repo_command::RepoCommand;
use crate::cli_out::print_in_cli::print_release;
use crate::cli_out::print_in_cli::print_repos;
use crate::cli_out::print_in_cli::print_url;
use crate::cli_parse::handle_commands::handle_issue::handle_issue_command;
use crate::git_utils::comments;
use crate::git_utils::releases;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;
use crate::git_utils::repos;
use octorust::{self, Client};
use std::process;

pub async fn handle_cli_command(args: Args, github_client: Client) {
    match args.command {
        CliCommand::Issue { subcommand } => {
            handle_issue_command(github_client, subcommand).await;
        }
        CliCommand::Comment { subcommand } => match subcommand {
            CommentCommand::Create { number, body } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let result = comments::create(&github_client, &repo_info, &number, &body).await;

                println!("{result}");
            }
        },

        CliCommand::Repo { subcommand } => match subcommand {
            RepoCommand::CreateForAuthenticatedUser {
                allow_auto_merge,
                allow_merge_commit,
                allow_rebase_merge,
                allow_squash_merge,
                auto_init,
                delete_branch_on_merge,
                description,
                gitignore_template,
                has_issues,
                has_projects,
                has_wiki,
                homepage,
                is_template,
                license_template,
                name,
                private,
            } => {
                let result = repos::create_for_authenticated_user(
                    &github_client,
                    allow_auto_merge,
                    allow_merge_commit,
                    allow_rebase_merge,
                    allow_squash_merge,
                    auto_init,
                    delete_branch_on_merge,
                    &description,
                    &gitignore_template,
                    has_issues,
                    has_projects,
                    has_wiki,
                    &homepage,
                    is_template,
                    &license_template,
                    &name,
                    private,
                )
                .await;

                println!("{result}");
            }

            RepoCommand::CreateInOrg {
                allow_auto_merge,
                allow_merge_commit,
                allow_rebase_merge,
                allow_squash_merge,
                auto_init,
                delete_branch_on_merge,
                description,
                gitignore_template,
                has_issues,
                has_projects,
                has_wiki,
                homepage,
                is_template,
                license_template,
                name,
                org,
                team_name,
                visibility,
            } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(org), Some(name)) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let result = repos::create_in_org(
                    &github_client,
                    repo_info,
                    allow_auto_merge,
                    allow_merge_commit,
                    allow_rebase_merge,
                    allow_squash_merge,
                    auto_init,
                    delete_branch_on_merge,
                    &description,
                    &gitignore_template,
                    has_issues,
                    has_projects,
                    has_wiki,
                    &homepage,
                    is_template,
                    &license_template,
                    &team_name,
                    Some(visibility.0),
                )
                .await;

                print_url(result, "New repo");
            }

            RepoCommand::GetAllFromOrg {
                org,
                order,
                sort_value,
                type_value,
            } => {
                let all_repos = repos::get_all_from_org(
                    &github_client,
                    &org,
                    order.0,
                    type_value.0,
                    sort_value.0,
                )
                .await;

                print_repos(all_repos, org, "org");
            }

            RepoCommand::CreateUsingTemplate {
                template_owner,
                template_name,
                name,
                owner,
                description,
                private,
                include_all_branches,
            } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(name))
                {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let template_info: RepoInfo =
                    match RepoInfo::new(Repo::Input, Some(template_owner), Some(template_name)) {
                        Ok(rep) => rep,
                        Err(message) => {
                            eprintln!("Error: {message}");
                            process::exit(1);
                        }
                    };

                let result = repos::create_using_template(
                    &github_client,
                    template_info,
                    repo_info,
                    &description,
                    include_all_branches,
                    private,
                )
                .await;

                print_url(result, "New repo");
            }

            RepoCommand::CreateFork { org, name, owner } => {
                let fork_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(name))
                {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let result = repos::create_fork(&github_client, &org, fork_info).await;

                println!("{result}");
            }

            RepoCommand::GetAllFromUser {
                owner,
                order,
                sort_value,
                type_value,
            } => {
                let result = repos::get_all_from_user(
                    &github_client,
                    owner.clone(),
                    type_value.0,
                    sort_value.0,
                    order.0,
                )
                .await;

                print_repos(result, owner, "user");
            }
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
