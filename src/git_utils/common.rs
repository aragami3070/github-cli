use std::{
    io::{self, ErrorKind},
    process,
};

use octorust::types::PullsUpdateReviewRequest;
use octorust::Client;

pub fn url_to_vars(url: &String) -> Result<(String, String), io::Error> {
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

pub async fn create_comment(
    github_client: &Client,
    repo_info: &String,
    issue_number: &i64,
    body: &String,
) -> String {
    let (owner, repo) = match url_to_vars(repo_info) {
        Ok(info) => info,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

    let request = PullsUpdateReviewRequest { body: body.clone() };

    let comment = github_client
        .issues()
        .create_comment(&owner.trim(), &repo.trim(), issue_number.clone(), &request)
        .await;

    return match comment {
        Ok(_) => "Comment create successed".to_string(),
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
