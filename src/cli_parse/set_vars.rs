use std::io;

use octorust::types::{self, State};

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
        return  None;
    } else {
        return Some(some_string);
    }
}
