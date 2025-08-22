use std::error::Error;
use std::fmt;

/// Errors returned by the tried print result
#[derive(Debug, PartialEq, Eq)]
pub struct PrintError {
    kind: PrintErrorKind,
    description: String,
}

/// Type of PrintError
#[derive(Debug, PartialEq, Eq)]
enum PrintErrorKind {
    /// Github response was wrong
    BadResponse,
}

impl Error for PrintError {}

impl fmt::Display for PrintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PrintErrorKind::BadResponse => {
                write!(
                    f,
                    "tried print, but the github response turned out to be invalid.\nDescription: {}",
                    &self.description
                )
            }
        }
    }
}

use octorust::types::{
    Issue, IssueComment, IssueSimple, MinimalRepository, PullRequestReviewComment, Release,
};

use crate::cli_in::set_vars::IssuesListStates;

pub fn print_release(result: Release) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!(" Release tag: {}", result.tag_name);
    println!(" Release id: {}", result.id);
    println!(" Release title: {}", result.name);
    println!(" Release body: {}", result.body);
    println!(" Release tag_commit: {}", result.target_commitish);
    println!(" Release url: {}", result.url);
    println!(" Release upload_url: {}", result.upload_url);
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
}

pub fn print_issues(list_issues: Vec<IssueSimple>, state: IssuesListStates, numb_of_page: i64) {
    println!(
        " {} {} Issues from {} page:",
        list_issues.len(),
        state.0,
        numb_of_page
    );
    println!();
    for issue in list_issues {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!(" Issue {}: {};", issue.number, issue.title);
        println!(" Body: {}", issue.body);
        println!(" labels:");
        for label in issue.labels {
            println!("   {}: {}", label.name, label.description);
        }
        match issue.created_at {
            Some(time) => {
                println!(" Created at: {}", time);
            }
            None => {}
        };
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
}

pub fn print_issue(issue: Issue) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!(" Issue {}: {};", issue.number, issue.title);
    println!(" State: {}", issue.state);
    println!(" Body: {}", issue.body);
    println!(" labels:");
    for label in issue.labels {
        match label.labels_data() {
            Some(data) => {
                println!("   Name: {}", data.name);
                println!("   Description: {}", data.description);
            }
            None => {}
        }
    }
    match issue.created_at {
        Some(time) => {
            println!(" Created at: {}", time);
        }
        None => {}
    };
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
}

pub fn print_url(result: String, description: &str) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!(" {} : {}", description, result.replace(" ", "-"));
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
}

pub fn print_repos(repos: Vec<MinimalRepository>, owner: String, owner_type: &str) {
    println!(" Found {} repos in {} {}", repos.len(), owner, owner_type);

    for repo in repos {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!(" Repo {}: {}", repo.id, repo.full_name);
        println!(" Language: {}", repo.language);
        println!(" Url: {}", repo.url);
        println!(" Description: {}", repo.description);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
}

pub fn print_comments(list_comments: Vec<IssueComment>) -> Result<(), Box<dyn Error>> {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!(" Comments:");
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    for comment in list_comments {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!(
            " Who create: {}",
            match comment.user {
                Some(u) => u.login,
                None => {
                    return Err(Box::new(PrintError {
                        kind: PrintErrorKind::BadResponse,
                        description: "User who create comment not find".to_string(),
                    }));
                }
            }
        );
        println!(" Body: {}", comment.body);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
    Ok(())
}

pub fn print_review_comments(
    list_comments: Vec<PullRequestReviewComment>,
) -> Result<(), Box<dyn Error>> {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!(" Review comments:");
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    for comment in list_comments {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!(
            " Who create: {}",
            match comment.user {
                Some(u) => u.login,
                None => {
                    return Err(Box::new(PrintError {
                        kind: PrintErrorKind::BadResponse,
                        description: "User who create comment not find".to_string(),
                    }));
                }
            }
        );
        println!(" Body: {}", comment.body);
        println!(" For line: {}", comment.line);
        println!(" In file: {}", comment.path);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
    Ok(())
}
