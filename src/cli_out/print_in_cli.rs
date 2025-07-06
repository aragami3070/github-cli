use octorust::types::{IssueSimple, MinimalRepository, Release};

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

pub fn print_issues(list_issues: Vec<IssueSimple>, state: String, numb_of_page: i64) {
    println!(
        " {} {} Issues from {} page:",
        list_issues.len(),
        state,
        numb_of_page
    );
    println!();
    for issue in list_issues {
        println!("╭────────────────────────────────────────────────────────────────────────────────────────────────");
        println!("│Issue {}: {};", issue.number, issue.title);
        println!("│Body: {}", issue.body);
        println!("╰────────────────────────────────────────────────────────────────────────────────────────────────");
    }
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
