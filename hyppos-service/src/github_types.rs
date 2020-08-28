use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Repo {
    pub owner: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitTree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitInfo {
    pub tree: CommitTree,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitDetails {
    pub commit: CommitInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BranchDetails {
    pub name: String,
    pub commit: CommitInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Directory {
    pub path: String,
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub path: String,
    pub sha: String,
    pub size: usize,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum DirEntry {
    #[serde(rename = "tree")]
    Dir {
        #[serde(flatten)]
        dir: Directory,
    },
    #[serde(rename = "blob")]
    File {
        #[serde(flatten)]
        file: File,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct DirectoryListing {
    #[serde(rename = "tree")]
    pub items: Vec<DirEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Blob {
    pub content: String,
}

pub trait DirectoryUrl {
    fn get_directory_url(&self) -> &str;
}

impl DirectoryUrl for &CommitTree {
    #[inline]
    fn get_directory_url(&self) -> &str {
        &self.url
    }
}

impl DirectoryUrl for &CommitInfo {
    #[inline]
    fn get_directory_url(&self) -> &str {
        &self.tree.url
    }
}

impl DirectoryUrl for &CommitDetails {
    #[inline]
    fn get_directory_url(&self) -> &str {
        &self.commit.tree.url
    }
}

impl DirectoryUrl for &Directory {
    #[inline]
    fn get_directory_url(&self) -> &str {
        &self.url
    }
}

impl DirectoryUrl for &str {
    #[inline]
    fn get_directory_url(&self) -> &str {
        self
    }
}
