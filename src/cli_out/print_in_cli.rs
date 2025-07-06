use octorust::types::Release;

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
