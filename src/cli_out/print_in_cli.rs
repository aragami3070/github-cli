use std::process;

use octorust::types::{
    Issue, IssueComment, IssueSimple, MinimalRepository, PullRequestReviewComment, Release,
};

use crate::cli_in::set_vars::IssuesListStates;

pub fn print_release(result: Release) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!("│Release tag: {}", result.tag_name);
    println!("│Release id: {}", result.id);
    println!("│Release title: {}", result.name);
    println!("│Release body: {}", result.body);
    println!("│Release tag_commit: {}", result.target_commitish);
    println!("│Release url: {}", result.url);
    println!("│Release upload_url: {}", result.upload_url);
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
        println!("│Issue {}: {};", issue.number, issue.title);
        println!("│Body: {}", issue.body);
        println!("│Time: {}", issue.timeline_url);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
}

pub fn print_issue(issue: Issue) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!("│Issue {}: {};", issue.number, issue.title);
    println!("│State: {}", issue.state);
    println!("│labels:");
    for label in issue.labels {
        match label.labels_data() {
            Some(data) => {
                println!("│  Name: {}", data.name);
                println!("│  Description: {}", data.description);
            }
            None => {}
        }
    }
    println!("│Body: {}", issue.body);
    match issue.created_at {
        Some(time) => {
            println!("│Created at: {}", time);
        }
        None => {}
    };
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
}

pub fn print_url(result: String, description: &str) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!("│{} : {}", description, result.replace(" ", "-"));
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
}

pub fn print_repos(repos: Vec<MinimalRepository>, owner: String, owner_type: &str) {
    println!(" Found {} repos in {} {}", repos.len(), owner, owner_type);

    for repo in repos {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!("│Repo {}: {}", repo.id, repo.full_name);
        println!("│Language: {}", repo.language);
        println!("│Url: {}", repo.url);
        println!("│Description: {}", repo.description);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
}

pub fn print_comments(list_comments: Vec<IssueComment>) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!("│Comments:");
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    for comment in list_comments {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!(
            "│Who create: {}",
            match comment.user {
                Some(u) => u.login,
                None => {
                    eprintln!("Bad response from github");
                    process::exit(1);
                }
            }
        );
        println!("│Body: {}", comment.body);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
}

pub fn print_review_comments(list_comments: Vec<PullRequestReviewComment>) {
    println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
    println!("│Review comments:");
    println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    for comment in list_comments {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!(
            "│Who create: {}",
            match comment.user {
                Some(u) => u.login,
                None => {
                    eprintln!("Bad response from github");
                    process::exit(1);
                }
            }
        );
        println!("│Body: {}", comment.body);
        println!("│For line: {}", comment.line);
        println!("│In file: {}", comment.path);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
}
