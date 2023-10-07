use std::error::Error;
use url::Url;
use mongodb::bson::DateTime;
use reqwest;


fn get_path_segments_from_url(remote_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = Url::parse(&remote_url)?;

    let path_segments: Vec<String> = match url.path_segments() {
        Some(segments) => segments.map(ToString::to_string).collect(),
        None => Vec::new(),
    };

    // https://github.com/{REPO_OWNER}/{REPO_NAME}
    assert_eq!(2, path_segments.len());

    return Ok(path_segments); 

}

fn get_repo_owner_from_url(remote_url: &String) -> Result<String, Box<dyn Error>> {
    let path_segments = get_path_segments_from_url(remote_url)?;
    return Ok(path_segments[0].to_string());
}

fn get_repo_name_from_url(remote_url: &String) -> Result<String, Box<dyn Error>> {
    let path_segments = get_path_segments_from_url(remote_url)?;
    return Ok(path_segments[1].to_string());
}

pub async fn check_remote(remote_url: &String) -> Result<(), Box<dyn Error>> {
    // let repo_owner = get_repo_owner_from_url(&remote_url)?;
    // let repo_name = get_repo_name_from_url(&remote_url)?;

    // TODO finish checking remote

    return Ok(());
}

pub async fn get_last_activity(remote_url: &String) -> Result<(), Box<dyn Error>>{
    println!("got here");
    let repo_owner = get_repo_owner_from_url(remote_url)?;
    let repo_name = get_repo_name_from_url(remote_url)?;


    let body = reqwest::get(format!("https://api.github.com/repos/{repo_owner}/{repo_name}/activity?per_page=1"))
        .await?
        .text()
        .await?;
    println!("body = {:?}", body);

    return Ok(());
}