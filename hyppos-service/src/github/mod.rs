pub mod types;
use self::types::{
    Blob, Branch, BranchDetails, CommitDetails, DirectoryListing, DirectoryUrl, Repo,
};

use reqwest::header::{self, HeaderMap, HeaderValue, InvalidHeaderValue};
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

pub struct GithubClient {
    client: reqwest::Client,
    base_url: Url,
}

impl GithubClient {
    pub fn with_baseurl(
        base_url: Url,
        token: impl std::fmt::Display,
    ) -> Result<Self, InvalidHeaderValue> {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("OAuth {}", token))?,
        );
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            )),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            // Fails only on serious problems like unavailable TLS backend
            .unwrap();
        Ok(Self { client, base_url })
    }

    pub fn new(token: impl std::fmt::Display) -> Result<Self, InvalidHeaderValue> {
        Self::with_baseurl(
            // Should never panic, because address is hardcoded
            Url::parse("https://api.github.com/").unwrap(),
            token,
        )
    }

    pub async fn list_branches(&self, repo: &Repo) -> Result<Vec<Branch>, Error> {
        Ok(self
            .client
            .get(
                self.base_url
                    .join(&format!("repos/{}/{}/branches", repo.owner, repo.name))?,
            )
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_branch(&self, repo: &Repo, name: &str) -> Result<BranchDetails, Error> {
        Ok(self
            .client
            .get(self.base_url.join(&format!(
                "repos/{}/{}/branches/{}",
                repo.owner, repo.name, name
            ))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_commit_details(&self, url: &str) -> Result<CommitDetails, Error> {
        Ok(self.client.get(url).send().await?.json().await?)
    }

    pub async fn get_commit_details_by_hash(
        &self,
        repo: &Repo,
        hash: &str,
    ) -> Result<CommitDetails, Error> {
        Ok(self
            .client
            .get(self.base_url.join(&format!(
                "repos/{}/{}/commits/{}",
                repo.owner, repo.name, hash
            ))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn list_directory(&self, url: impl DirectoryUrl) -> Result<DirectoryListing, Error> {
        Ok(self
            .client
            .get(url.get_directory_url())
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_file_contents(&self, url: &str) -> Result<Vec<u8>, Error> {
        let blob: Blob = self.client.get(url).send().await?.json().await?;
        // Why, GitHub, why?
        base64::decode(&blob.content.replace('\n', "")).map_err(Error::from)
    }
}
