use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Commit {
    pub sha: String,
    /// This is the pull request which this commit merged in.
    #[serde(default)]
    pub pr: Option<u32>,
    pub time: chrono::DateTime<chrono::Utc>,
}

/// This provides the master-branch Rust commits which should have accompanying
/// bors artifacts available.
///
/// The first commit returned (at index 0) is the most recent, the last is the
/// oldest.
///
/// Specifically, this is the last 168 days of bors commits.
///
/// Note that this does not contain try commits today, so it should not be used
/// to validate hashes or expand them generally speaking. This may also change
/// in the future.
pub async fn master_commits() -> Result<Vec<Commit>, Box<dyn std::error::Error + Sync + Send>> {
    let response = reqwest::get("https://triage.rust-lang.org/bors-commit-list").await?;
    Ok(response.json().await?)
}
