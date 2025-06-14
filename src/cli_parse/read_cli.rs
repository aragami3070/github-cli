use std::io;

use clap::{Parser, Subcommand};
use octorust::types;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Get list of issues
    IssuesList {
        /// The user that created the issues (optional)
        #[clap(long, short, default_value = "")]
        creator: String,
        /// Can be the name of a user. Pass in `none` for issues with no assigned user, and `*` for issues assigned to any user (optional)
        #[clap(long, short, default_value = "none")]
        assignee: String,
        /// Indicates the state of the issues to return. Can be either `open`, `closed`, or `all` (optional)
        #[clap(long, short, default_value = "open")]
        state: String,
        /// A list of comma separated label names. Example: `bug,ui,@high` (optional)
        #[clap(long, short, default_value = "")]
        labels: String,
        /// Page number of the results to fetch (optional)
        #[clap(long, short, default_value = "1")]
        numb_of_page: i64,
        /// Results on page (max 100) (optional)
        #[clap(long, short, default_value = "30", value_parser = clap::value_parser!(i64).range(1..=100))]
        iss_on_page: i64,
    },
}

pub fn set_state(state: String) -> Result<types::IssuesListState, io::Error> {
    if state == "open" {
        return Ok(types::IssuesListState::Open);
    } else if state == "all" {
        return Ok(types::IssuesListState::All);
    } else if state == "closed" {
        return Ok(types::IssuesListState::Closed);
    } else {
		return Err(io::Error::new(io::ErrorKind::InvalidData, "Bad input. State can be only 'open', 'all', 'closed'"));
    }
}
