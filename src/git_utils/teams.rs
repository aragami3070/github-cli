use std::error::Error;

use octorust::{types::FullTeam, Client};

pub async fn get_id(
    github_client: &Client,
    org: &str,
    name: &str,
) -> Result<FullTeam, Box<dyn Error>> {
    let team = github_client.teams().get_by_name(org, name).await;

    match team {
        Ok(t) => Ok(t.body),
        Err(err) => Err(Box::new(err)),
    }
}
