use std::{io, process::Command};

pub struct RepoInfo {
    owner: String,
    name: String,
    url: String,
    ssh: String,
}

impl RepoInfo {
    fn parse_github_url(url: &str) -> Result<(String, String), io::Error> {
        let parts: Vec<&str> = url
            .trim() // Delete spaces
            .trim_end_matches(".git") // Delete .git in end
            .split(&['/', ':'][..]) // Split by '/', ':'
            .collect();
        if parts.len() > 2 {
            let owner = parts[parts.len() - 2];
            let repo = parts[parts.len() - 1];
            return Ok((owner.to_string(), repo.to_string()));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Can't parse string",
            ));
        }
    }

    /// Get repo from .git in current directory
    fn get_current_repo(mut self) -> Result<Self, io::Error> {
        // Creating a command to search for a link to a remote repository
        let git_link = Command::new("git")
            .args(&["remote", "get-url", "origin"])
            .output()?;

        // Git repository was not found in this directory
        if !git_link.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Git repository not found in this directory",
            ));
        }

        // Reading the result of the command execution into a string
        let repo_url = String::from_utf8_lossy(&git_link.stdout).trim().to_string();

        (self.owner, self.name) = Self::parse_github_url(&repo_url)?;

        Self::set_url(&mut self);
        Self::set_ssh(&mut self);

        return Ok(self);
    }

    fn set_url(&mut self) {
        self.url = String::from("https://github.com/");
        self.url.push_str(self.owner.trim());
        self.url.push_str("/");
        self.url.push_str(self.name.trim());
        self.url.push_str("/");
    }

    fn set_ssh(&mut self) {
        self.ssh = String::from("git@github.com:");
        self.ssh.push_str(self.owner.trim());
        self.ssh.push_str("/");
        self.ssh.push_str(self.name.trim());
        self.ssh.push_str(".git");
    }

    pub fn get_owner(&self) -> String {
        return self.owner.replace("-", " ").clone();
    }

    pub fn get_name(&self) -> String {
        return self.name.replace("-", " ").clone();
    }

    pub fn get_url(&self) -> String {
        return self.url.clone();
    }

    pub fn get_ssh(&self) -> String {
        return self.ssh.clone();
    }

    pub fn new(owner: Option<String>, name: Option<String>) -> Result<RepoInfo, io::Error> {
        if owner.is_none() && name.is_none() {
            let new_repo = RepoInfo {
                owner: String::new(),
                name: String::new(),
                url: String::new(),
                ssh: String::new(),
            };
            return Self::get_current_repo(new_repo);
        } else if owner.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Not found repo owner",
            ));
        } else if name.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Not found repo name",
            ));
        } else {
            let mut new_repo = RepoInfo {
                owner: owner.unwrap().trim_start().trim_end().replace(" ", "-"),
                name: name.unwrap().trim_start().trim_end().replace(" ", "-"),
                url: String::new(),
                ssh: String::new(),
            };
            Self::set_url(&mut new_repo);
            Self::set_ssh(&mut new_repo);

            return Ok(new_repo);
        }
    }
}
