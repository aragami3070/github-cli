use std::{
    io::{self, ErrorKind},
    option, process,
};

use octorust::types::{
    self, IssuesCreateRequest, IssuesCreateRequestLabelsOneOf, IssuesUpdateRequest, State,
    TitleOneOf,
};
use octorust::Client;

fn url_to_vars(url: &String) -> Result<(String, String), io::Error> {
    if let Some(pos) = url.find('/') {
        let owner = url[..pos].to_string();
        let repo = url[pos + 1..].to_string();
        return Ok((owner, repo));
    } else {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "This url isn't valid",
        ));
    }
}

pub async fn get_issues_list(
    github_client: &Client,
    repo_info: &String,
    creator: &String,
    assignee: &String,
    state: &types::IssuesListState,
    labels: &String,
    numb_of_page: &i64,
    iss_on_page: &i64,
) -> Vec<types::IssueSimple> {
    let sort = types::IssuesListSort::Created;

    let (owner, repo) = match url_to_vars(repo_info) {
        Ok(info) => info,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let issues = github_client
        .issues()
        .list_for_repo(
            owner.trim(),
            repo.trim(),
            "",
            state.clone(),
            assignee,
            creator,
            "",
            labels,
            sort,
            types::Order::Noop,
            None,
            iss_on_page.clone(),
            numb_of_page.clone(),
        )
        .await;

    return match issues {
        Ok(info) => info.body,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

fn get_create_request(
    title: &String,
    body: &String,
    assignees: &Vec<String>,
    labels: &Vec<String>,
) -> IssuesCreateRequest {
    let new_title = TitleOneOf::String(title.clone());

    let new_labels = labels
        .into_iter()
        .map(|s| IssuesCreateRequestLabelsOneOf::String(s.into()))
        .collect();

    IssuesCreateRequest {
        title: new_title,
        body: body.clone(),
        assignee: String::new(),
        assignees: if assignees.is_empty() {
            Vec::new()
        } else {
            assignees.clone()
        },
        labels: new_labels,
        milestone: None,
    }
}

pub async fn create_issue(
    github_client: &Client,
    repo_info: &String,
    title: &String,
    body: &String,
    assignees: &Vec<String>,
    labels: &Vec<String>,
) -> String {
    let (owner, repo) = match url_to_vars(repo_info) {
        Ok(info) => info,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let request = get_create_request(title, body, assignees, labels);

    let new_issue = github_client
        .issues()
        .create(owner.trim(), repo.trim(), &request)
        .await;

    return match new_issue {
        Ok(_) => "Success".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

fn get_update_request(
    title: Option<&String>,
    body: Option<&String>,
    assignees: Option<&Vec<String>>,
    labels: Option<&Vec<String>>,
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

pub async fn close_issue(
    github_client: &Client,
    repo_info: &String,
    issue_number: &i64,
    comment: &String,
) -> String {
    let (owner, repo) = match url_to_vars(repo_info) {
        Ok(info) => info,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let request = get_update_request(None, None, None, None, &State::Closed);

    let close = github_client
        .issues()
        .update(owner.trim(), repo.trim(), issue_number.clone(), &request)
        .await;

    return match close {
        Ok(_) => "Success".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
