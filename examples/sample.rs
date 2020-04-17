#[tokio::main]
async fn main() {
    env_logger::init();
    let list = rustc_artifacts::master_commits().await.unwrap();
    eprintln!("first: {:#?}", list.first());
    eprintln!("last: {:#?}", list.last());
    eprintln!("commits: {}", list.len());
}
