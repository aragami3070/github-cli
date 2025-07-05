use std::process;

use octorust::types::{
    self, IssuesCreateRequest, IssuesCreateRequestLabelsOneOf, IssuesUpdateRequest, State,
    TitleOneOf,
};
use octorust::Client;

use crate::git_utils::comments;
use crate::git_utils::repo_info::RepoInfo;

pub async fn get_list(
    github_client: &Client,
    repo_info: &RepoInfo,
    creator: &String,
    assignee: &String,
    state: &types::IssuesListState,
    labels: &String,
    numb_of_page: &i64,
    iss_on_page: &i64,
) -> Vec<types::IssueSimple> {
    let sort = types::IssuesListSort::Created;

    let issues = github_client
        .issues()
        .list_for_repo(
            &repo_info.get_owner().trim(),
            &repo_info.get_name().trim(),
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

pub async fn create(
    github_client: &Client,
    repo_info: RepoInfo,
    title: &String,
    body: &String,
    assignees: &Vec<String>,
    labels: &Vec<String>,
) -> String {
    let request = get_create_request(title, body, assignees, labels);

    let new_issue = github_client
        .issues()
        .create(
            &repo_info.get_owner().trim(),
            &repo_info.get_name().trim(),
            &request,
        )
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

pub async fn close(
    github_client: &Client,
    repo_info: RepoInfo,
    issue_number: &i64,
    comment: &String,
) -> String {
    if comment != "" {
        let new_comment = comments::create(github_client, &repo_info, issue_number, comment).await;
        println!("{new_comment}");
    }

    let request = get_update_request(None, None, None, None, &State::Closed);

    let close = github_client
        .issues()
        .update(
            &repo_info.get_owner().trim(),
            &repo_info.get_name().trim(),
            issue_number.clone(),
            &request,
        )
        .await;

    return match close {
        Ok(_) => "Success".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}

pub async fn update(
    github_client: &Client,
    repo_info: RepoInfo,
    issue_number: &i64,
    title: Option<&String>,
    body: Option<&String>,
    assignees: &Vec<String>,
    labels: &Vec<String>,
    state: &State,
) -> String {
    let request = get_update_request(title, body, Some(assignees), Some(labels), state);

    let update_iss = github_client
        .issues()
        .update(
            &repo_info.get_owner().trim(),
            &repo_info.get_name().trim(),
            issue_number.clone(),
            &request,
        )
        .await;

    return match update_iss {
        Ok(_) => "Success".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
