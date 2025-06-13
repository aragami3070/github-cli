use clap::{Parser, Subcommand};
#[derive(Parser)]
pub struct Args {
	#[clap(subcommand)]
	pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    // Получить список всех issues
	IssuesList,
}
