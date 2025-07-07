use std::io;
use std::io::ErrorKind;
use std::str::FromStr;

use octorust::types::{self, Order, ReposListOrgSort, ReposListUserType, State};
use octorust::types::{ReposCreateInOrgRequestVisibility, ReposListOrgType};


#[derive(Debug, Clone)]
pub struct Orders(pub Order);

impl FromStr for Orders {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Orders(Order::Asc)),
            "desc" => Ok(Orders(Order::Desc)),
            _ => Err("Bad input. Order can be only 'asc' or 'desc'".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReposListOrgSorts(pub ReposListOrgSort);

impl FromStr for ReposListOrgSorts {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
        "created" => Ok(ReposListOrgSorts(ReposListOrgSort::Created)),
        "fullname" => Ok(ReposListOrgSorts(ReposListOrgSort::FullName)),
        "pushed" => Ok(ReposListOrgSorts(ReposListOrgSort::Pushed)),
        "updated" => Ok(ReposListOrgSorts(ReposListOrgSort::Updated)),
            _ => Err("Bad input. Sort can be only 'created', 'fullname', 'pushed' or 'updated'".to_string()),
        }
    }
}

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

pub fn set_repos_list_user_type(type_value: &String) -> Result<ReposListUserType, io::Error> {
    return match type_value.trim() {
        "all" => Ok(ReposListUserType::All),
        "member" => Ok(ReposListUserType::Member),
        "owner" => Ok(ReposListUserType::Owner),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            "Bad input. Type can be only 'all', 'member' or 'owner'",
        )),
    };
}

