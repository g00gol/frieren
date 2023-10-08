mod db;
mod github;
mod fern;

use tokio;
use std::error::Error;
use github::GithubFile;
use chrono::{DateTime, Utc};

async fn handle_repo(repo: db::Repo) -> Result<(), Box<dyn Error>> {
    let ref repo_origin = repo.repo_origin;
    let ref repo_oid = repo._id;
    let ref repo_hash = repo.hash;
    let last_updated = github::get_last_activity(&repo_origin).await?;
    let dt: DateTime<Utc> = DateTime::<Utc>::from_timestamp(last_updated, 0).expect("invalid timestamp");
    
    let file: GithubFile = match github::get_fern_file(&repo_origin, Some(&"cli".to_string())).await {
        Ok(_file) => _file,
        Err(_) => github::get_fern_file(&repo_origin, None).await?
    };

    let new_hash = github::get_fern_hash_from_github(&file);
    match github::is_fern_file_hash_equal(&new_hash, &repo_hash) {
        true => {},
        false => {
            // code
            println!("Code!");
        }
    }

    let stars = github::get_star_count(&repo_origin).await?;
    github::get_languages(&repo_origin).await?;

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
