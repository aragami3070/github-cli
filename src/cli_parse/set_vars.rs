use std::str::FromStr;

use octorust::types::{IssuesListState, Order, ReposListOrgSort, ReposListUserType, State};
use octorust::types::{ReposCreateInOrgRequestVisibility, ReposListOrgType};

#[derive(Debug, Clone)]
pub struct Orders(pub Order);

impl FromStr for Orders {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Self(Order::Asc)),
            "desc" => Ok(Self(Order::Desc)),
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
            "created" => Ok(Self(ReposListOrgSort::Created)),
            "fullname" => Ok(Self(ReposListOrgSort::FullName)),
            "pushed" => Ok(Self(ReposListOrgSort::Pushed)),
            "updated" => Ok(Self(ReposListOrgSort::Updated)),
            _ => Err(
                "Bad input. Sort can be only 'created', 'fullname', 'pushed' or 'updated'"
                    .to_string(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReposListUserTypes(pub ReposListUserType);

impl FromStr for ReposListUserTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self(ReposListUserType::All)),
            "member" => Ok(Self(ReposListUserType::Member)),
            "owner" => Ok(Self(ReposListUserType::Owner)),
            _ => Err("Bad input. Type can be only 'all', 'member' or 'owner'".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReposListOrgTypes(pub ReposListOrgType);

impl FromStr for ReposListOrgTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
        "all" => Ok(Self(ReposListOrgType::All)),
		"forks" => Ok(Self(ReposListOrgType::Forks)),
		"internal" => Ok(Self(ReposListOrgType::Internal)),
		"member" => Ok(Self(ReposListOrgType::Member)),
		"private" => Ok(Self(ReposListOrgType::Private)),
		"public" => Ok(Self(ReposListOrgType::Public)),
		"sources" => Ok(Self(ReposListOrgType::Sources)),
            _ => Err("Bad input. Type can be only 'all', 'forks', 'internal', 'member', 'private', 'public' or 'sources'".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Visibilities(pub ReposCreateInOrgRequestVisibility);

impl FromStr for Visibilities {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self(ReposCreateInOrgRequestVisibility::Noop)),
            "private" => Ok(Self(ReposCreateInOrgRequestVisibility::Private)),
            "public" => Ok(Self(ReposCreateInOrgRequestVisibility::Public)),
            "internal" => Ok(Self(ReposCreateInOrgRequestVisibility::Internal)),
            _ => Err(
                "Bad input. Visibility can be only '', 'public', 'private' or 'internal'"
                    .to_string(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct States(pub State);

impl FromStr for States {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(Self(State::Open)),
            "closed" => Ok(Self(State::Closed)),
            _ => Err("Bad input. State can be only 'open' or 'closed'".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IssuesListStates(pub IssuesListState);

impl FromStr for IssuesListStates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(Self(IssuesListState::Open)),
            "closed" => Ok(Self(IssuesListState::Closed)),
            "all" => Ok(Self(IssuesListState::All)),
            _ => Err("Bad input. State can be only 'open', 'all', 'closed'".to_string()),
        }
    }
}

impl ToString for IssuesListStates {
    fn to_string(&self) -> String {
        match self.0 {
            IssuesListState::Open => String::from("open"),
            IssuesListState::Closed => String::from("closed"),
            IssuesListState::All => String::from("all"),
            IssuesListState::FallthroughString => String::from("FallthroughString"),
        }
    }
}

pub fn set_option_string(some_string: &String) -> Option<&String> {
    return match some_string.trim() {
        "None" => None,
        _ => Some(some_string),
    };
}
