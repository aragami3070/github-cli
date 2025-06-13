use std::{
    io::{self, ErrorKind},
    process,
};

use octorust::Client;

use crate::octorust::issues::Issues;

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

pub fn get_issues_list(github_client: &Client, repo_info: &String) {
    let state = octorust::types::IssuesListState::Open;
    let sort = octorust::types::IssuesListSort::Created;

    let (owner, repo) = match url_to_vars(repo_info) {
        Ok(info) => info,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };

	println!("Owner: {owner}; Repo: {repo}");

}
