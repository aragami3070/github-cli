use std::{
    io::{self, ErrorKind},
    process,
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
) -> Vec<octorust::types::IssueSimple> {
    let state = octorust::types::IssuesListState::Open;
    let sort = octorust::types::IssuesListSort::Created;

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
            state,
            "*",
            "",
            "",
            "",
            sort,
            octorust::types::Order::Noop,
            None,
            100,
            1,
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
