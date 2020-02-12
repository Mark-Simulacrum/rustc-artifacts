use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct Commit {
    pub sha: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct CommitResponse {
    sha: String,
    commit: CommitResponseCommit,
}

#[derive(Deserialize)]
struct CommitResponseCommit {
    committer: Committer,
}

#[derive(Deserialize)]
struct Committer {
    date: chrono::DateTime<chrono::Utc>,
}

/// This provides the master-branch Rust commits
/// which have accompanying bors artifacts available.
///
/// Specifically, this is the last 168 days of bors commits.
///
/// Note that this does not contain try commits today, so it should not be used
/// to validate hashes or expand them generally speaking. This may also change
/// in the future.
///
/// We currently use the GitHub API to retrieve this list; the GH token
/// allows you to make this request more often. If it is not provided the
/// requests issued will be unauthenticated.
pub async fn master_commits(
    client: &Client,
    token: Option<&str>,
) -> Result<Vec<Commit>, Box<dyn std::error::Error + Sync + Send>> {
    log::trace!("token: {:?}", token);
    let mut commits = Vec::new();

    // YYYY-MM-DDTHH:MM:SSZ
    let earliest_available = chrono::Utc::now() - chrono::Duration::days(168);
    let url = format!(
        "https://api.github.com/repos/rust-lang/rust/commits?author=bors&since={}&per_page=100",
        earliest_available.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    );
    log::trace!("requesting {}", url);
    let mut request = client
        .get(&url)
        .header(USER_AGENT, "rust-lang-commit-lister");

    if let Some(token) = token {
        request = request.header("Authorization", &format!("token {}", token));
    }

    let request = request.build()?;

    let mut next_request = Some(request);
    while let Some(request) = next_request.take() {
        // We expect to be able to clone the request, as it has no streaming
        // parts.
        let mut request_copy = request.try_clone().unwrap();
        let response = client.execute(request).await?;
        if let Some(link) = response.headers().get("Link").and_then(|l| l.to_str().ok()) {
            log::trace!("Considering link header {}", link);
            // format:
            // <url>; rel="next", ...
            if let Some(part) = link.split(',').find(|e| e.ends_with(r#"rel="next""#)) {
                if let Some(url) = part.get(1..link.find(r#">; rel="next""#).unwrap()) {
                    log::trace!("resolved URL: {}", url);
                    *request_copy.url_mut() = reqwest::Url::parse(url)?;
                    next_request = Some(request_copy);
                }
            }
        }
        let commit_part = response.text().await?;
        let commit_part: Vec<CommitResponse> = serde_json::from_str(&commit_part).map_err(|e| {
            log::error!("{:?}", commit_part);
            e
        })?;
        commits.extend(commit_part);
    }

    Ok(commits
        .into_iter()
        .rev()
        .map(|c: CommitResponse| Commit {
            sha: c.sha,
            date: c.commit.committer.date,
        })
        .collect())
}
