use clap::Parser;
use std::process;

mod cli_parse;
mod git_utils;

use crate::cli_parse::read_cli::Args;
use crate::git_utils::get_repo_info::get_current_repo;

fn main() {
    let repo_info: String = match get_current_repo() {
        Ok(repo_url) => repo_url,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let args: Args = Args::parse();
}
