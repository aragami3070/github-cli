use clap::Parser;
use octorust::{self, auth::Credentials, Client};
use std::process;

mod cli_parse;
mod git_utils;

use crate::cli_parse::comment_command::CommentCommand;
use crate::cli_parse::issue_command::IssueCommand;
use crate::cli_parse::read_cli::Args;
use crate::cli_parse::read_cli::CliCommand;
use crate::cli_parse::release_command::ReleaseCommand;
use crate::cli_parse::repo_command::RepoCommand;
use crate::cli_parse::set_vars::set_issues_list_state;
use crate::cli_parse::set_vars::set_option_string;
use crate::cli_parse::set_vars::set_order;
use crate::cli_parse::set_vars::set_repos_list_org_sort;
use crate::cli_parse::set_vars::set_repos_list_org_type;
use crate::cli_parse::set_vars::set_state;
use crate::cli_parse::set_vars::set_visibility;
use crate::git_utils::comments;
use crate::git_utils::issues;
use crate::git_utils::releases;
use crate::git_utils::repo_info::RepoInfo;
use crate::git_utils::repos;

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let github_token = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("Error: GITHUB_TOKEN enviroment variable not set");
            process::exit(1);
        }
    };

    let github_client: Client =
        Client::new("github-cli".to_string(), Credentials::Token(github_token))
            .expect("Failed to create Github client");

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
                let repo_info: RepoInfo = match RepoInfo::new(None, None) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let inp_state = match set_issues_list_state(&state) {
                    Ok(res) => res,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let list_issues = issues::get_list(
                    &github_client,
                    repo_info,
                    &creator,
                    &assignee,
                    &inp_state,
                    &labels,
                    &numb_of_page,
                    &iss_on_page,
                )
                .await;
                println!(
                    "{} {} Issues from {} page:",
                    list_issues.len(),
                    state,
                    numb_of_page
                );
                println!();
                for issue in list_issues {
                    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
                    println!("│Issue {}: {};", issue.number, issue.title);
                    println!("│Body: {}", issue.body);
                    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
                }
            }

            IssueCommand::Create {
                title,
                body,
                assignees,
                labels,
            } => {
                let repo_info: RepoInfo = match RepoInfo::new(None, None) {
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
                let repo_info: RepoInfo = match RepoInfo::new(None, None) {
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
                let repo_info: RepoInfo = match RepoInfo::new(None, None) {
                    Ok(rep) => rep,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let new_state = match set_state(&state) {
                    Ok(s) => s,
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

                let new_body: Option<&String> = set_option_string(&body);
                let new_title: Option<&String> = set_option_string(&title);

                let result = issues::update(
                    &github_client,
                    repo_info,
                    &number,
                    new_title,
                    new_body,
                    &assignees_list,
                    &labels_list,
                    &new_state,
                )
                .await;

                println!("{result}");
            }
        },

        CliCommand::Comment { subcommand } => match subcommand {
            CommentCommand::Create { number, body } => {
                let repo_info: RepoInfo = match RepoInfo::new(None, None) {
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
                let new_visibility = match set_visibility(&visibility) {
                    Ok(v) => v,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let result = repos::create_in_org(
                    &github_client,
                    &org,
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
                    &team_name,
                    Some(new_visibility),
                )
                .await;

                println!("{result}");
            }
            RepoCommand::GetAllFromOrg {
                org,
                order,
                sort_value,
                type_value,
            } => {
                let new_order = match set_order(&order) {
                    Ok(o) => o,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let new_sort = match set_repos_list_org_sort(&sort_value) {
                    Ok(o) => o,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };
                let new_type = match set_repos_list_org_type(&type_value) {
                    Ok(o) => o,
                    Err(message) => {
                        eprintln!("Error: {message}");
                        process::exit(1);
                    }
                };

                let all_repos =
                    repos::get_all_from_org(&github_client, &org, new_order, new_type, new_sort)
                        .await;

                println!("Found {} repos in {} org", all_repos.len(), org);

                for repo in all_repos {
                    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
                    println!("│Repo {}: {}", repo.id, repo.full_name);
                    println!("│Language: {}", repo.language);
                    println!("│Url: {}", repo.url);
                    println!("│Description: {}", repo.description);
                    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
                }
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
                let result = repos::create_using_template(
                    &github_client,
                    &template_owner,
                    &template_name,
                    &name,
                    &owner,
                    &description,
                    include_all_branches,
                    private,
                )
                .await;

                println!("{result}");
            }
            RepoCommand::CreateFork { org, name, owner } => {
                let result = repos::create_fork(&github_client, &org, &owner, &name).await;

                println!("{result}");
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
                let result = releases::create(
                    &github_client,
                    &owner,
                    &repo,
                    body,
                    discussion_category_name,
                    draft,
                    &name,
                    prerelease,
                    &tag_name,
                    target_commitish,
                )
                .await;

                println!("{result}");
            }
        },
    }
}
