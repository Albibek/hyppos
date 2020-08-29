use crate::github_types::{
    Blob, Branch, BranchDetails, CommitDetails, DirectoryListing, DirectoryUrl, Repo, RepoDetails,
    User,
};

use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

impl Repo {
    pub fn new(owner: impl Into<String>, name: impl Into<String>) -> Repo {
        Repo {
            owner: owner.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed constructing URL: {0}")]
    ParseUrlError(#[from] url::ParseError),
    #[error("Failed requesting API: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed decoding Base64: {0}")]
    Base64Error(#[from] base64::DecodeError),
}

#[derive(Clone)]
pub struct GithubClient {
    http: reqwest::Client,
    base_url: Url,
}

pub struct GithubClientForToken<'a> {
    client: &'a GithubClient,
    token_header: String,
}

impl GithubClient {
    pub fn with_baseurl(base_url: Url) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            )),
        );
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            // Fails only on serious problems like unavailable TLS backend
            .unwrap();
        Self { http, base_url }
    }

    pub fn new() -> Self {
        Self::with_baseurl(
            // Should never panic, because address is hardcoded
            Url::parse("https://api.github.com/").unwrap(),
        )
    }

    pub fn for_token<'a>(&'a self, token: &str) -> GithubClientForToken<'a> {
        GithubClientForToken {
            client: self,
            token_header: format!("token {}", token),
        }
    }
}

impl GithubClientForToken<'_> {
    async fn get<T: DeserializeOwned>(&self, url: impl reqwest::IntoUrl) -> Result<T, Error> {
        self.client
            .http
            .get(url)
            .header(header::AUTHORIZATION, &self.token_header)
            .send()
            .await?
            .json()
            .await
            .map_err(Error::from)
    }

    async fn get_relative<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = self.client.base_url.join(path)?;
        self.get(url).await
    }

    pub async fn list_branches(&self, repo: &Repo) -> Result<Vec<Branch>, Error> {
        self.get_relative(&format!("repos/{}/{}/branches", repo.owner, repo.name))
            .await
    }

    pub async fn get_branch(&self, repo: &Repo, name: &str) -> Result<BranchDetails, Error> {
        self.get_relative(&format!(
            "repos/{}/{}/branches/{}",
            repo.owner, repo.name, name
        ))
        .await
    }

    pub async fn get_commit_details(&self, url: &str) -> Result<CommitDetails, Error> {
        self.get(url).await
    }

    pub async fn get_commit_details_by_hash(
        &self,
        repo: &Repo,
        hash: &str,
    ) -> Result<CommitDetails, Error> {
        self.get_relative(&format!(
            "repos/{}/{}/commits/{}",
            repo.owner, repo.name, hash
        ))
        .await
    }

    pub async fn list_directory(&self, url: impl DirectoryUrl) -> Result<DirectoryListing, Error> {
        self.get(url.get_directory_url()).await
    }

    pub async fn get_file_contents(&self, url: &str) -> Result<Vec<u8>, Error> {
        let blob: Blob = self.get(url).await?;
        // Why, GitHub, why?
        base64::decode(&blob.content.replace('\n', "")).map_err(Error::from)
    }

    pub async fn get_user(&self) -> Result<User, Error> {
        self.get_relative("user").await
    }

    pub async fn get_user_repos(&self, username: &str) -> Result<Vec<RepoDetails>, Error> {
        self.get_relative(&format!("users/{}/repos", username))
            .await
    }

    pub async fn get_own_user_repos(
        &self,
        username: &str,
    ) -> Result<impl Iterator<Item = RepoDetails>, Error> {
        Ok(self
            .get_user_repos(username)
            .await?
            .into_iter()
            .filter(|repo| !(repo.private || repo.fork)))
    }
}
