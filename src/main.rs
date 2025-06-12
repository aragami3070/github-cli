mod git_utils;

fn main() {
	match git_utils::get_repo_info::get_current_repo() {
		Ok(repo_url) => println!("{repo_url}"),
		Err(e) => println!("{e}"),
	}
}
