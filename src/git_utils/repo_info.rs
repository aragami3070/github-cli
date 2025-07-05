use std::{io, process::Command, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoOwner(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoName(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoUrl(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoSsh(String);

impl RepoOwner {
    pub fn trim(&self) -> &str {
		return self.0.trim();
	}
}

impl RepoName {
    pub fn trim(&self) -> &str {
		return self.0.trim();
	}
}

impl RepoUrl {
    pub fn push_str(&mut self, s: &str) {
		self.0.push_str(s);
	}
}

impl RepoSsh {
    pub fn push_str(&mut self, s: &str) {
		self.0.push_str(s);
	}
}

impl FromStr for RepoOwner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("Repo owner cannot be empty".to_string())
        } else if s.contains('/') {
            Err("Repo owner cannot contain '/'".to_string())
        } else {
            Ok(RepoOwner(s.to_string()))
        }
    }
}

impl FromStr for RepoName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("Repo name cannot be empty".to_string())
        } else if s.contains('/') {
            Err("Repo name cannot contain '/'".to_string())
        } else {
            Ok(RepoName(s.to_string()))
        }
    }
}

pub struct RepoInfo {
    owner: RepoOwner,
    name: RepoName,
    url: RepoUrl,
    ssh: RepoSsh,
}

impl RepoInfo {
    fn parse_github_url(url: &str) -> Result<(RepoOwner, RepoName), io::Error> {
        let parts: Vec<&str> = url
            .trim() // Delete spaces
            .trim_end_matches(".git") // Delete .git in end
            .split(&['/', ':'][..]) // Split by '/', ':'
            .collect();
        if parts.len() > 2 {
            let owner = parts[parts.len() - 2];
            let repo = parts[parts.len() - 1];
            return Ok((RepoOwner(owner.to_string()), RepoName(repo.to_string())));
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
        self.url = RepoUrl(String::from("https://github.com/"));
        self.url.push_str(self.owner.trim());
        self.url.push_str("/");
        self.url.push_str(self.name.trim());
        self.url.push_str("/");
    }

    fn set_ssh(&mut self) {
        self.ssh = RepoSsh(String::from("git@github.com:"));
        self.ssh.push_str(self.owner.trim());
        self.ssh.push_str("/");
        self.ssh.push_str(self.name.trim());
        self.ssh.push_str(".git");
    }

    pub fn get_owner(&self) -> String {
        return self.owner.clone().0;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone().0;
    }

    pub fn get_release_url(&self, tag: &String) -> String {
        let mut release_url = self.url.clone();
        release_url.push_str("releases/tag/");
        release_url.push_str(tag.trim());
        return release_url.0;
    }

    pub fn get_ssh(&self) -> String {
        return self.ssh.clone().0;
    }

    pub fn new(owner: Option<RepoOwner>, name: Option<RepoName>) -> Result<RepoInfo, io::Error> {
        if owner.is_none() && name.is_none() {
            let new_repo = RepoInfo {
                owner: RepoOwner(String::new()),
                name: RepoName(String::new()),
                url: RepoUrl(String::new()),
                ssh: RepoSsh(String::new()),
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
                owner: RepoOwner(owner.unwrap().0.trim_start().trim_end().to_string()),
                name: RepoName(name.unwrap().0.trim_start().trim_end().to_string()),
                url: RepoUrl(String::new()),
                ssh: RepoSsh(String::new()),
            };
            Self::set_url(&mut new_repo);
            Self::set_ssh(&mut new_repo);

            return Ok(new_repo);
        }
    }
}
