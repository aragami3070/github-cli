use std::{io, process::Command};

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

    return Ok(repo_url);
}
