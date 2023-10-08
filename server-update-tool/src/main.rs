mod db;
mod github;
mod fern;

use tokio;
use std::error::Error;
use github::GithubFile;
use chrono::{DateTime, Utc};
use std::collections::hash_set::HashSet;
use log::{debug};
use env_logger;

async fn handle_repo(repo: db::Repo) -> Result<(), Box<dyn Error>> {
    let ref repo_origin = repo.repo_origin;
    let ref repo_hash = repo.hash;
    
    let ref mut new_repo = repo.clone();

    let repo_metadata = github::get_repo_metadata(&repo_origin).await?;

    debug!("Starting to handle repo {}", repo.name.unwrap());

    let last_updated = github::get_last_activity(&repo_metadata).await?;
    let dt_last_updated: DateTime<Utc> = DateTime::<Utc>::from_timestamp(last_updated, 0).expect("invalid timestamp");
    
    // We put the hack in hackathon
    new_repo.date_created = DateTime::<Utc>::from_timestamp(github::get_created_at_time(&repo_metadata).await?, 0).expect("Invalid timestamp");

    let file: GithubFile = match github::get_fern_file(&repo_origin, Some(&"cli".to_string())).await {
        Ok(_file) => _file,
        Err(_) => github::get_fern_file(&repo_origin, None).await? // TODO if this still fails, delete DB entry
    };

    new_repo.hash = Some(github::get_fern_hash_from_github(&file));
    
    let langs: Vec<String> = github::get_languages(&repo_origin).await?;

    match github::is_fern_file_hash_equal(&new_repo.hash.as_ref().unwrap(), &repo_hash) {
        true => {},
        false => {

            let content = fern::read_b64_content(file.content.trim().to_string()).unwrap(); // we put the hack in hackathon
            new_repo.name = Some(content.name);
            new_repo.description = Some(content.description);
            let mut technologies = content.technologies.clone();

            println!("techs: {:?}", technologies);
            langs.iter().for_each(|x| technologies.push(x.to_string()));
            technologies.sort();
            technologies.dedup();

            new_repo.technologies = Some(technologies);
            new_repo.difficulty = Some(content.difficulty.into());
            new_repo.recommended_issue_labels = Some(content.recommended_issue_labels);

        }
    }

    let stars = github::get_star_count(&repo_metadata).await?;
    new_repo.stars = Some(stars);
    new_repo.last_updated = dt_last_updated;
    new_repo.recommended_issues_count = Some(github::count_recommended_issues(&repo_origin, &new_repo.recommended_issue_labels.as_ref().unwrap()).await?);

    db::update_repo(new_repo).await?;

    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

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
