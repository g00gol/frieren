use std::error::Error;
use url::Url;
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub _self: String,

    pub git: String,
    pub html: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: usize,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,

    #[serde(rename = "type")] 
    pub _type: String,
    pub content: String,
    pub encoding: String,

    #[serde(flatten)]
    pub _links: Links
}

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

pub async fn get_fern_file(remote_uri: &String, branch_name: &String) -> Result<GithubFile, Box<dyn Error>> {
    let repo_owner = get_repo_owner_from_url(&remote_uri)?;
    let repo_name = get_repo_name_from_url(&remote_uri)?;

    let github_uri = format!("https://api.github.com/repos/{}/{}/contents/open-source.fern?ref={}", repo_owner, repo_name, branch_name);

    let file: GithubFile = reqwest::Client::new()
        .get(github_uri)
        .send()
        .await?
        .json()
        .await?;

    println!("{:?}", file);
    return Ok(file); 
}

pub async fn get_last_activity(remote_url: &String) -> Result<(), Box<dyn Error>>{
    let repo_owner = get_repo_owner_from_url(remote_url)?;
    let repo_name = get_repo_name_from_url(remote_url)?;


    let body = reqwest::get(format!("https://api.github.com/repos/{repo_owner}/{repo_name}/activity?per_page=1"))
        .await?
        .text()
        .await?;
    println!("body = {:?}", body);

    return Ok(());
}
