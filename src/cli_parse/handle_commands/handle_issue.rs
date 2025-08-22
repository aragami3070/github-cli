use octorust::{self, Client};
use std::error::Error;

use crate::cli_in::issue_command::IssueCommand;
use crate::cli_in::set_vars::IssuesListStates;
use crate::cli_in::set_vars::States;
use crate::cli_out::fuzzy_select::choose_issue;
use crate::cli_out::print_in_cli::print_comments;
use crate::cli_out::print_in_cli::print_issue;
use crate::cli_out::print_in_cli::print_issues;
use crate::cli_out::print_in_cli::print_simple_issue;
use crate::git_utils::comments;
use crate::git_utils::issues;
use crate::git_utils::repo_info::Repo;
use crate::git_utils::repo_info::RepoInfo;
use crate::git_utils::repo_info::{RepoName, RepoOwner};

pub async fn handle_issue_command(
    github_client: Client,
    subcommand: IssueCommand,
) -> Result<(), Box<dyn Error>> {
    match subcommand {
        IssueCommand::List {
            owner,
            repo,
            creator,
            assignee,
            state,
            labels,
            numb_of_page,
            iss_on_page,
        } => {
            handle_list(
                github_client,
                owner,
                repo,
                creator,
                assignee,
                state,
                labels,
                numb_of_page,
                iss_on_page,
            )
            .await?;
            Ok(())
        }

        IssueCommand::Get {
            owner,
            repo,
            number,
        } => {
            handle_get(github_client, owner, repo, number).await?;
            Ok(())
        }

        IssueCommand::Create {
            owner,
            repo,
            title,
            body,
            assignees,
            labels,
        } => {
            handle_create(github_client, owner, repo, title, body, assignees, labels).await?;
            Ok(())
        }

        IssueCommand::Close {
            owner,
            repo,
            number,
            comment,
        } => {
            handle_close(github_client, owner, repo, number, comment).await?;
            Ok(())
        }

        IssueCommand::Update {
            owner,
            repo,
            number,
            title,
            body,
            assignees,
            state,
            labels,
        } => {
            handle_update(
                github_client,
                owner,
                repo,
                title,
                body,
                number,
                labels,
                assignees,
                state,
            )
            .await?;
            Ok(())
        }
    }
}

async fn handle_list(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    creator: String,
    assignee: String,
    state: IssuesListStates,
    labels: String,
    numb_of_page: i64,
    iss_on_page: i64,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
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
    .await?;

    print_issues(list_issues, state, numb_of_page);
    Ok(())
}

async fn handle_get(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    number: i64,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };

    let result = issues::get(&github_client, &repo_info, number).await?;

    let list_comments = comments::get_all(&github_client, &repo_info, &number).await?;

    print_issue(result);
    print_comments(list_comments)?;
    Ok(())
}

async fn handle_get_form_list(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    creator: String,
    assignee: String,
    state: IssuesListStates,
    labels: String,
    numb_of_page: i64,
    iss_on_page: i64,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
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
    .await?;

    let choosed_issue = choose_issue(list_issues)?;

    if let Some(ch_i) = choosed_issue {
        let list_comments = comments::get_all(&github_client, &repo_info, &ch_i.number).await?;

        print_simple_issue(ch_i);
        print_comments(list_comments)?;
    } else {
        println!("Issue not choosed or not find");
    }

    Ok(())
}

async fn handle_create(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    title: String,
    body: String,
    assignees: String,
    labels: String,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };

    let labels_list: Vec<String> = labels.split(",").map(|s| s.to_string()).collect();
    let assignees_list: Vec<String> = assignees.split(",").map(|s| s.to_string()).collect();

    let result = issues::create(
        &github_client,
        repo_info,
        &title,
        &body,
        &assignees_list,
        &labels_list,
    )
    .await?;

    println!("{result}");
    Ok(())
}

async fn handle_close(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    number: i64,
    comment: String,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
    };

    let result = issues::close(&github_client, repo_info, &number, &comment).await?;

    println!("{result}");
    Ok(())
}

async fn handle_update(
    github_client: Client,
    owner: Option<RepoOwner>,
    repo: Option<RepoName>,
    title: Option<String>,
    body: Option<String>,
    number: i64,
    labels: Option<String>,
    assignees: Option<String>,
    state: States,
) -> Result<(), Box<dyn Error>> {
    let repo_info = match owner {
        Some(_) => RepoInfo::new(Repo::Input, owner, repo)?,
        None => RepoInfo::new(Repo::Current, None, None)?,
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
        title,
        body,
        &assignees_list,
        &labels_list,
        &state.0,
    )
    .await?;

    println!("{result}");
    Ok(())
}
