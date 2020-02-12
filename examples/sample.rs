#[tokio::main]
async fn main() {
    env_logger::init();
    let client = reqwest::Client::new();
    let list =
        rustc_artifacts::master_commits(&client, std::env::var("GITHUB_API_TOKEN").ok().as_deref())
            .await
            .unwrap();
    eprintln!("first: {:#?}", list.first());
    eprintln!("last: {:#?}", list.last());
    eprintln!("commits: {}", list.len());
}
