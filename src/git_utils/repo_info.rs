use std::{io, process::Command, str::FromStr};

pub enum Repo {
    Current,
    Input,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn create_repo_info(
        owner: Option<RepoOwner>,
        name: Option<RepoName>,
    ) -> Result<RepoInfo, io::Error> {
        if owner.is_none() {
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
                owner: RepoOwner(owner.unwrap().0.trim().to_string()),
                name: RepoName(name.unwrap().0.trim().to_string()),
                url: RepoUrl(String::new()),
                ssh: RepoSsh(String::new()),
            };
            Self::set_url(&mut new_repo);
            Self::set_ssh(&mut new_repo);

            return Ok(new_repo);
        }
    }

    pub fn new(
        type_repo: Repo,
        owner: Option<RepoOwner>,
        name: Option<RepoName>,
    ) -> Result<RepoInfo, io::Error> {
        match type_repo {
            Repo::Current => {
                let new_repo = RepoInfo {
                    owner: RepoOwner(String::new()),
                    name: RepoName(String::new()),
                    url: RepoUrl(String::new()),
                    ssh: RepoSsh(String::new()),
                };
                return Self::get_current_repo(new_repo);
            }
            Repo::Input => {
                return Self::create_repo_info(owner, name);
            }
        }
    }
}

#[cfg(test)]
mod repo_info_tests {
    use super::*;
    use paste::paste;
    use rstest::rstest;

    // Macros for Repo owner and name tests
    macro_rules! repo_main_field_valid_tests {
        ($type: ident, $type_name: literal) => {
            paste! {
                // Tests valid cases
                #[rstest]
                #[case("aragami3070", "aragami3070")]
                #[case("SE-legacy", "SE-legacy")]
                #[case("The Drot Team", "The Drot Team")]
                fn [<valid_ $type:snake>](#[case] input: &str, #[case] expected: &str) {
                    assert_eq!(
                        $type::from_str(input).unwrap(),
                        $type(expected.to_string())
                    );
                }

                // Tests invalid cases
                #[rstest]
                #[case("", concat!($type_name, " cannot be empty"))]
                #[case("owner/repo", concat!($type_name, " cannot contain '/'"))]
                #[case("/owner", concat!($type_name, " cannot contain '/'"))]
                #[case("/owner/repo/", concat!($type_name, " cannot contain '/'"))]
                fn [<invalid_ $type:snake>](#[case] input: &str, #[case] error: &str) {
                    assert_eq!($type::from_str(input).unwrap_err(), error);
                }
            }
        };
    }

    // RepoOwner tests
    repo_main_field_valid_tests!(RepoOwner, "Repo owner");
    // RepoName tests
    repo_main_field_valid_tests!(RepoName, "Repo name");

    macro_rules! repo_links_field_valid_test {
        ($type: ident, $field_name: ident, $start_link: literal, $end_link: literal) => {
            paste! {
                // Tests valid cases
                #[rstest]
                #[case(
                    "aragami3070",
                    "github-cli",
                    concat!($start_link, "aragami3070/github-cli", $end_link)
                )]
                #[case(
                    " SE-legacy     ",
                    "kg-exam-4sem",
                    concat!($start_link, "SE-legacy/kg-exam-4sem", $end_link)
                )]
                #[case(
                    "The Drot Team",
                    "      Trash-Hack-Back-end",
                    concat!($start_link, "The-Drot-Team/Trash-Hack-Back-end", $end_link)
                )]
                fn [<valid_ $type:snake>](#[case] owner: &str, #[case] name: &str, #[case] expected: &str) {
                    let repo = RepoInfo::new(
                        Repo::Input,
                        Some(RepoOwner(owner.to_string())),
                        Some(RepoName(name.to_string())),
                    );
                    assert_eq!(repo.unwrap().$field_name.0.replace(" ", "-"), expected);
                }
            }
        };
    }

    // RepoUrl valid test
    repo_links_field_valid_test!(RepoUrl, url, "https://github.com/", "/");

    // RepoSsh valid test
    repo_links_field_valid_test!(RepoSsh, ssh, "git@github.com:", ".git");

    // Tests valid case
    #[test]
    fn valid_new_repo_info() {
        let repo_input = RepoInfo::new(
            Repo::Input,
            Some(RepoOwner("aragami3070".to_string())),
            Some(RepoName("github-cli".to_string())),
        );
        let repo_current = RepoInfo::new(Repo::Current, None, None);
        assert_eq!(repo_input.unwrap(), repo_current.unwrap());
    }

}
