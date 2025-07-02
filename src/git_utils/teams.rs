use std::process;

use octorust::{types::FullTeam, Client};

pub async fn get_id(github_client: &Client, org: &String, name: &String) -> FullTeam {
    let team = github_client
        .teams()
        .get_by_name(org.trim(), name.trim())
        .await;
    return match team {
        Ok(t) => t.body,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(1);
        }
    };
}
