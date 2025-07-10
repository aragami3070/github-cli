use crate::cli_in::comment_command::CommentCommand;
use crate::cli_in::issue_command::IssueCommand;
use crate::cli_in::read_cli::Args;
use crate::cli_in::read_cli::CliCommand;
use crate::cli_in::release_command::ReleaseCommand;
use crate::cli_in::repo_command::RepoCommand;
use crate::cli_out::print_in_cli::print_issues;
use crate::cli_out::print_in_cli::print_release;
use crate::cli_out::print_in_cli::print_repos;
use crate::cli_out::print_in_cli::print_url;
use crate::git_utils::comments;
use crate::git_utils::issues;
use crate::git_utils::releases;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;
use crate::git_utils::repos;
use octorust::{self, Client};
use std::process;

pub async fn handle_cli_command(args: Args, github_client: Client) {
    match args.command {
        CliCommand::Issue { subcommand } => match subcommand {
            IssueCommand::List {
                creator,
                assignee,
                state,
                labels,
                numb_of_page,
                iss_on_page,
            } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let list_issues = issues::get_list(
                    &github_client,
                    &repo_info,
                    &creator,
                    &assignee,
                    &state.0,
                    &labels,
                    &numb_of_page,
                    &iss_on_page,
                )
                .await;

                print_issues(list_issues, state, numb_of_page);
            }

            IssueCommand::Create {
                title,
                body,
                assignees,
                labels,
            } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let labels_list: Vec<String> = labels.split(",").map(|s| s.to_string()).collect();
                let assignees_list: Vec<String> =
                    assignees.split(",").map(|s| s.to_string()).collect();

                let result = issues::create(
                    &github_client,
                    repo_info,
                    &title,
                    &body,
                    &assignees_list,
                    &labels_list,
                )
                .await;

                println!("{result}");
            }

            IssueCommand::Close { number, comment } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let result = issues::close(&github_client, repo_info, &number, &comment).await;

                println!("{result}");
            }

            IssueCommand::Update {
                number,
                title,
                body,
                assignees,
                state,
                labels,
            } => {
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Current, None, None) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let labels_list: Vec<String> = match labels {
                    Some(l) => l.split(",").map(|s| s.to_string()).collect(),
                    None => Vec::new(),
                };
                let assignees_list: Vec<String> = match assignees {
                    Some(a) => a.split(",").map(|s| s.to_string()).collect(),
                    None => Vec::new(),
                };

                let result = issues::update(
                    &github_client,
                    repo_info,
                    &number,
                    Some(title),
                    Some(body),
                    &assignees_list,
                    &labels_list,
                    &state.0,
                )
                .await;

                println!("{result}");
            }
        },

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
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(name)) {
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
                let fork_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(name)) {
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
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo)) {
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
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo)) {
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
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo)) {
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
                let repo_info: RepoInfo = match RepoInfo::new(Repo::Input, Some(owner), Some(repo)) {
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
