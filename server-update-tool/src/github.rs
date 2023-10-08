use std::error::Error;
use url::Url;
use reqwest;
use reqwest::header::{ACCEPT, USER_AGENT};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;
// use mongodb::bson::DateTime;
use chrono::{DateTime};
use mongodb::bson::oid::ObjectId;

use md5;
use crate::db;

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

pub async fn get_fern_file(remote_uri: &String, branch_name: Option<&String>) -> Result<GithubFile, Box<dyn Error>> {
    let repo_owner = get_repo_owner_from_url(&remote_uri)?;
    let repo_name = get_repo_name_from_url(&remote_uri)?;

    let github_uri = match branch_name {
        Some(_branch) => format!("https://api.github.com/repos/{}/{}/contents/open-source.fern?ref={}", repo_owner, repo_name, _branch),
        None => format!("https://api.github.com/repos/{}/{}/contents/open-source.fern", repo_owner, repo_name)
    };

    let file: GithubFile = reqwest::Client::new()
        .get(github_uri)
        .header(USER_AGENT, "Frieren API")
        .send()
        .await?
        .json()
        .await?;

    return Ok(file); 
}

pub async fn update_last_activity(remote_url: &String) -> Result<i64, Box<dyn Error>>{
    let repo_owner = get_repo_owner_from_url(remote_url)?;
    let repo_name = get_repo_name_from_url(remote_url)?;

    let github_uri = format!("https://api.github.com/repos/{repo_owner}/{repo_name}/activity?per_page=1");

    let json: serde_json::Value = reqwest::Client::new()
        .get(github_uri)
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        .send()
        .await?
        .json()
        .await?;

    let time_string = json[0]["timestamp"].to_string();
    let timestamp_str = time_string.trim_matches('"');
    let timestamp = DateTime::parse_from_rfc3339(timestamp_str)?;

    return Ok(timestamp.timestamp());
}

fn get_fern_hash_from_github(file: &GithubFile) -> String {
    return format!("{:x}", md5::compute(&file.content));
}

pub async fn fern_file_job(file: &GithubFile, repo: &db::Repo) -> Result<(), Box<dyn Error>> {
    let github_fern_file_hash = get_fern_hash_from_github(file);
    return match is_fern_file_hash_equal(github_fern_file_hash, repo) {
        true => Ok(()),
        false => {
            // TODO implement all fern file DB updates

            return Ok(());
        }
    }
}

fn is_fern_file_hash_equal(hash: String, repo: &db::Repo) -> bool {
    // TODO should make md5 compute a separate function and pass that in accordingly
    return match &repo.hash {
        Some(_hash) => _hash.to_string() == hash,
        None => false
    }
}

pub async fn get_star_count(remote_url: &String) -> Result<u64, Box<dyn Error>>{
    let repo_owner = get_repo_owner_from_url(remote_url)?;
    let repo_name = get_repo_name_from_url(remote_url)?;

    let uri = format!("https://api.github.com/repos/{}/{}", repo_owner, repo_name);

    let json_data: serde_json::Value = reqwest::Client::new()
        .get(uri)
        .header(ACCEPT, "application/vnd.github+json")
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        .send()
        .await?
        .json()
        .await?;
    let star_count: u64 = json_data.get("subscribers_count").unwrap().as_u64().unwrap();
    println!("star count = {}", star_count);
    
    return Ok(star_count);
}
