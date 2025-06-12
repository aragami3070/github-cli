use std::{io, process::Command};

fn parse_github_url(url: &str) -> Result<String, io::Error> {
    let parts: Vec<&str> = url
        .trim() // Убираем пробелы
        .trim_end_matches(".git") // Убираем .git в конце
        .split(&['/', ':'][..]) // Делим по '/', ':'
        .collect();
    if parts.len() > 2 {
        let owner = parts[parts.len() - 2];
        let repo = parts[parts.len() - 1];
        return Ok(format!("{owner}/{repo}"));
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Can't parse string",
        ));
    }
}

pub fn get_current_repo() -> Result<String, io::Error> {
    // Создаем команду для поиска ссылки на удаленный репозиторий
    let git_link = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .output()?;

    // Git репозиторий не найден в данной директории
    if !git_link.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Git repository not found in this directory",
        ));
    }

    // Считываем результат выполнения команды в строку
    let repo_url = String::from_utf8_lossy(&git_link.stdout).trim().to_string();

	let parsed_url = parse_github_url(&repo_url)?;
    return Ok(parsed_url);
}
