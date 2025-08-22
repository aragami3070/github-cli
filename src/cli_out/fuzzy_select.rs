use std::error::Error;

use fzf_wrapped::{run_with_output, Border, Fzf};
use octorust::types::IssueSimple;

pub fn choose_issue(list_issues: Vec<IssueSimple>) -> Result<Option<IssueSimple>, Box<dyn Error>> {
    let issue_titles: Vec<String> = list_issues
        .iter()
        .map(|iss| format!("({}) Title: {}", iss.number, iss.title))
        .collect();

    let fzf = Fzf::builder()
        .border(Border::Rounded)
        .border_label("List issues")
        .header("Pick needed issue form list")
        .build()?;

    let choosed_issue = run_with_output(fzf, issue_titles);

    let issue_num = match choosed_issue {
        Some(ch_i) => {
            let index = match ch_i.find(')') {
                Some(c) => c,
                None => return Ok(None),
            };
            ch_i[1..index].parse::<i64>()
        }
        None => return Ok(None),
    }?;

    for issue in list_issues {
        if issue.number == issue_num {
            return Ok(Some(issue));
        }
    }
    Ok(None)
}
