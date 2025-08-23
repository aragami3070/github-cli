use std::error::Error;

use octorust::types::{
    self, Issue, IssuesCreateRequest, IssuesCreateRequestLabelsOneOf, IssuesUpdateRequest, State,
    TitleOneOf,
};
use octorust::Client;

use crate::cli_parse::entities::{ListIssueArgs, UpdateIssueArgs};
use crate::git_utils::comments;
use crate::git_utils::repo_info::RepoInfo;

pub async fn get(
    github_client: &Client,
    repo_info: &RepoInfo,
    issue_number: i64,
) -> Result<Issue, Box<dyn Error>> {
    let issue = github_client
        .issues()
        .get(&repo_info.get_owner(), &repo_info.get_name(), issue_number)
        .await?;

    Ok(issue.body)
}

pub async fn get_list(
    github_client: &Client,
    repo_info: &RepoInfo,
    command_args: &ListIssueArgs,
) -> Result<Vec<types::IssueSimple>, Box<dyn Error>> {
    let sort = types::IssuesListSort::Created;

    let issues = github_client
        .issues()
        .list_for_repo(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            "",
            command_args.state.0.to_owned(),
            &command_args.assignee,
            &command_args.creator,
            "",
            &command_args.labels,
            sort,
            types::Order::Noop,
            None,
            command_args.iss_on_page,
            command_args.numb_of_page,
        )
        .await;

    match issues {
        Ok(info) => Ok(info.body),
        Err(er) => Err(Box::new(er)),
    }
}

fn get_create_request(
    title: &str,
    body: &str,
    assignees: &[String],
    labels: &[String],
) -> IssuesCreateRequest {
    let new_title = TitleOneOf::String(title.to_owned());

    let new_labels = labels
        .iter()
        .map(|s| IssuesCreateRequestLabelsOneOf::String(s.into()))
        .collect();

    IssuesCreateRequest {
        title: new_title,
        body: body.to_owned(),
        assignee: String::new(),
        assignees: if assignees.is_empty() {
            Vec::new()
        } else {
            assignees.to_owned()
        },
        labels: new_labels,
        milestone: None,
    }
}

pub async fn create(
    github_client: &Client,
    repo_info: RepoInfo,
    title: &str,
    body: &str,
    assignees: &[String],
    labels: &[String],
) -> Result<String, Box<dyn Error>> {
    let request = get_create_request(title, body, assignees, labels);

    let new_issue = github_client
        .issues()
        .create(&repo_info.get_owner(), &repo_info.get_name(), &request)
        .await;

    match new_issue {
        Ok(_) => Ok("Success".to_string()),
        Err(er) => Err(Box::new(er)),
    }
}

fn get_update_request(
    title: Option<String>,
    body: Option<String>,
    assignees: Option<&[String]>,
    labels: Option<&[String]>,
    state: &State,
) -> IssuesUpdateRequest {
    let new_title = title.map(|t| TitleOneOf::String(t.to_string()));

    let new_labels = match labels {
        Some(l) => l
            .iter()
            .map(|s| IssuesCreateRequestLabelsOneOf::String(s.to_string()))
            .collect(),
        None => Vec::new(),
    };

    IssuesUpdateRequest {
        title: new_title,
        body: body.map(|b| b.to_string()).unwrap_or_default(),
        assignee: String::new(),
        assignees: assignees.map(|a| a.to_vec()).unwrap_or_default(),
        labels: new_labels,
        milestone: None,
        state: Some(state.clone()),
    }
}

pub async fn close(
    github_client: &Client,
    repo_info: RepoInfo,
    issue_number: &i64,
    comment: &str,
) -> Result<String, Box<dyn Error>> {
    if comment.is_empty() {
        let new_comment =
            comments::create(github_client, &repo_info, issue_number, comment).await?;
        println!("{new_comment}");
    }

    let request = get_update_request(None, None, None, None, &State::Closed);

    let close = github_client
        .issues()
        .update(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            *issue_number,
            &request,
        )
        .await;

    match close {
        Ok(_) => Ok("Success".to_string()),
        Err(er) => Err(Box::new(er)),
    }
}

pub async fn update(
    github_client: &Client,
    repo_info: RepoInfo,
    command_args: UpdateIssueArgs,
    assignees: &[String],
    labels: &[String],
) -> Result<String, Box<dyn Error>> {
    let request = get_update_request(
        command_args.title,
        command_args.body,
        Some(assignees),
        Some(labels),
        &command_args.state.0,
    );

    let update_iss = github_client
        .issues()
        .update(
            &repo_info.get_owner(),
            &repo_info.get_name(),
            command_args.number,
            &request,
        )
        .await;

    match update_iss {
        Ok(_) => Ok("Success".to_string()),
        Err(er) => Err(Box::new(er)),
    }
}
