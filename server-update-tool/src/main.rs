mod db;
mod github;

use tokio;
use std::error::Error;

async fn handle_repo(repo: db::Repo) -> Result<(), Box<dyn Error>> {
    let repo_origin = repo.repo_origin;
    github::check_remote(&repo_origin).await?;
    github::get_last_activity(&repo_origin).await?;

    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cursor = db::get_repos().await?;

    while cursor.advance().await? {
        let res = cursor.deserialize_current();
        match res {
            Ok(repo) => {
                match handle_repo(repo).await {
                    Ok(_) => (),
                    Err(e) => println!("Failed to update DB object: {}", e)
                }
            },
            Err(e) => println!("Failed to deserialize db object: {}", e)
        }
    }

    return Ok(());
}