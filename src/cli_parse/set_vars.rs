use std::io;
use std::io::ErrorKind;
use std::process;

use crate::git_utils::get_repo_info::get_current_repo;
use octorust::types::{self, State};
use octorust::types::{ReposCreateInOrgRequestVisibility, ReposListOrgType};

pub fn set_issues_list_state(state: &String) -> Result<types::IssuesListState, io::Error> {
    if state == "open" {
        return Ok(types::IssuesListState::Open);
    } else if state == "all" {
        return Ok(types::IssuesListState::All);
    } else if state == "closed" {
        return Ok(types::IssuesListState::Closed);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Bad input. State can be only 'open', 'all', 'closed'",
        ));
    }
}

pub fn set_state(state: &String) -> Result<State, io::Error> {
    if state == "open" {
        return Ok(State::Open);
    } else if state == "closed" {
        return Ok(State::Closed);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Bad input. State can be only 'open' or 'closed'",
        ));
    }
}

pub fn set_option_string(some_string: &String) -> Option<&String> {
    if some_string == "None" {
        return None;
    } else {
        return Some(some_string);
    }
}

pub fn set_repo() -> String {
    return match get_current_repo() {
        Ok(repo_url) => repo_url,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };
}

pub fn set_visibility(visibility: &String) -> Result<ReposCreateInOrgRequestVisibility, io::Error> {
    if visibility == "" {
        return Ok(ReposCreateInOrgRequestVisibility::Noop);
    } else if visibility == "private" {
        return Ok(ReposCreateInOrgRequestVisibility::Private);
    } else if visibility == "public" {
        return Ok(ReposCreateInOrgRequestVisibility::Public);
    } else if visibility == "internal" {
        return Ok(ReposCreateInOrgRequestVisibility::Internal);
    } else {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Bad input. Visibility can be only '', 'public', 'private' or 'internal'",
        ));
    }
}

pub fn set_repos_list_org_type(type_value: &String) -> Result<ReposListOrgType, io::Error> {
    return match type_value.as_str() {
		"all" => Ok(ReposListOrgType::All),
		"forks" => Ok(ReposListOrgType::Forks),
		"internal" => Ok(ReposListOrgType::Internal),
		"member" => Ok(ReposListOrgType::Member),
		"private" => Ok(ReposListOrgType::Private),
		"public" => Ok(ReposListOrgType::Public),
		"sources" => Ok(ReposListOrgType::Sources),
		_ => Err(io::Error::new(
            ErrorKind::InvalidData,
            "Bad input. Type can be only 'all', 'forks', 'internal', 'member', 'private', 'public' or 'sources'",
        )),
	};
}
