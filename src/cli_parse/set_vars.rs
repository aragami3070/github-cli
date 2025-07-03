use std::io;
use std::io::ErrorKind;
use std::process;

use crate::git_utils::get_repo_info::get_current_repo;
use octorust::types::{self, Order, ReposListOrgSort, State};
use octorust::types::{ReposCreateInOrgRequestVisibility, ReposListOrgType};

pub fn set_issues_list_state(state: &String) -> Result<types::IssuesListState, io::Error> {
    return match state.trim() {
        "open" => Ok(types::IssuesListState::Open),
        "closed" => Ok(types::IssuesListState::Closed),
        "all" => Ok(types::IssuesListState::All),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Bad input. State can be only 'open', 'all', 'closed'",
        )),
    };
}

pub fn set_state(state: &String) -> Result<State, io::Error> {
    return match state.trim() {
        "open" => Ok(State::Open),
        "closed" => Ok(State::Closed),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Bad input. State can be only 'open' or 'closed'",
        )),
    };
}

pub fn set_option_string(some_string: &String) -> Option<&String> {
    return match some_string.trim() {
        "None" => None,
        _ => Some(some_string),
    };
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
    return match visibility.trim() {
        "" => Ok(ReposCreateInOrgRequestVisibility::Noop),
        "private" => Ok(ReposCreateInOrgRequestVisibility::Private),
        "public" => Ok(ReposCreateInOrgRequestVisibility::Public),
        "internal" => Ok(ReposCreateInOrgRequestVisibility::Internal),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            "Bad input. Visibility can be only '', 'public', 'private' or 'internal'",
        )),
    };
}

pub fn set_repos_list_org_type(type_value: &String) -> Result<ReposListOrgType, io::Error> {
    return match type_value.trim() {
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

pub fn set_repos_list_org_sort(sort_value: &String) -> Result<ReposListOrgSort, io::Error> {
    return match sort_value.trim() {
        "created" => Ok(ReposListOrgSort::Created),
        "fullname" => Ok(ReposListOrgSort::FullName),
        "pushed" => Ok(ReposListOrgSort::Pushed),
        "updated" => Ok(ReposListOrgSort::Updated),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            "Bad input. Sort can be only 'created', 'fullname', 'pushed' or 'updated'",
        )),
    };
}

pub fn set_order(order: &String) -> Result<Order, io::Error> {
    return match order.trim() {
        "asc" => Ok(Order::Asc),
        "desc" => Ok(Order::Desc),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            "Bad input. Order can be only 'asc' or 'desc'",
        )),
    };
}
