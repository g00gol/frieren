use std::error::Error;
use url::Url;
use reqwest;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
// use mongodb::bson::DateTime;
use chrono::{DateTime};
use mongodb::bson::oid::ObjectId;
use reqwest::Response;
use md5;
use crate::db;
use futures::future::{BoxFuture, FutureExt};
use std::{thread, time};
use log::{debug};

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
    assert_eq!(3, path_segments.len()); // 3rd element is ""

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

async fn get_request_wrapper(url: &String) -> Result<Response, Box<dyn Error>> {

    let sleep_duration = time::Duration::from_millis(5000);

    debug!("Attempting to make request to {}", url);

    for n in 1..3 { // we put the hack in hackathon
        let response = reqwest::Client::new()
            .get(url)
        .header(USER_AGENT, "Frieren API")
        .send()
        .await?;
        match response.status().as_u16() {
            403 => {
                debug!("Rate limiter. Sleeping {}ms", sleep_duration.as_millis());
                thread::sleep(sleep_duration);
            },
            200 => {  
                debug!("Successfully made request to {}", url);
                return Ok(response)
            } 
            _ => return Err("Error while querying URL".into())
        }
    }
    debug!("Failed to request {url}. Too many requests");

    return Err("Rate limiter too strong".into()); // We put the hack in hackathon    
}

pub async fn get_fern_file(remote_uri: &String, branch_name: Option<&String>) -> Result<GithubFile, Box<dyn Error>> {
    debug!("Getting fern file");
    let repo_owner = get_repo_owner_from_url(&remote_uri)?;
    let repo_name = get_repo_name_from_url(&remote_uri)?;

    let github_uri = match branch_name {
        Some(_branch) => format!("https://api.github.com/repos/{}/{}/contents/open-source.fern?ref={}", repo_owner, repo_name, _branch),
        None => format!("https://api.github.com/repos/{}/{}/contents/open-source.fern", repo_owner, repo_name)
    };

    let file: GithubFile = get_request_wrapper(&github_uri)
        .await?
        .json()
        .await?;

    debug!("Successfully got fern file"); 
    return Ok(file); 
}

pub async fn get_created_at_time(repo_metadata: &Value) -> Result<i64, Box<dyn Error>> {
    debug!("Getting created at time");

    let time_string = repo_metadata["created_at"].to_string();
    let timestamp_str = time_string.trim_matches('"');
    let timestamp = DateTime::parse_from_rfc3339(timestamp_str)?;

    debug!("Successfully got created at time");
    return Ok(timestamp.timestamp());

}

pub async fn get_last_activity(repo_metadata: &Value) -> Result<i64, Box<dyn Error>>{
    debug!("Getting created last activity time");

    let time_string = repo_metadata["updated_at"].to_string();
    let timestamp_str = time_string.trim_matches('"');
    let timestamp = DateTime::parse_from_rfc3339(timestamp_str)?;

    debug!("Successfully got last activity time");
    return Ok(timestamp.timestamp());

}

pub fn get_fern_hash_from_github(file: &GithubFile) -> String {
    return format!("{:x}", md5::compute(&file.content));
}

pub fn is_fern_file_hash_equal(hash: &String, old_hash: &Option<String>) -> bool {
    return match old_hash {
        Some(_hash) => _hash.to_string() == hash.to_string(),
        None => false
    }
}

pub async fn get_repo_metadata(remote_url: &String) -> Result<Value, Box<dyn Error>> {
    debug!("Getting repo metadata");
    let repo_owner = get_repo_owner_from_url(remote_url)?;
    let repo_name = get_repo_name_from_url(remote_url)?;

    let uri = format!("https://api.github.com/repos/{}/{}", repo_owner, repo_name);

    let json_data: serde_json::Value = get_request_wrapper(&uri)
        .await?
        .json()
        .await?;
    
    return Ok(json_data);
}

pub async fn get_star_count(repo_metadata: &Value) -> Result<u64, Box<dyn Error>>{

    debug!("Getting star count");

    let star_count: u64 = repo_metadata.get("subscribers_count").unwrap().as_u64().unwrap();
    
    debug!("Successfully got star count");
    return Ok(star_count);
}

pub async fn get_languages(remote_url: &String) -> Result<Vec<String>, Box<dyn Error>>{
    debug!("Getting languages");
    let repo_owner = get_repo_owner_from_url(remote_url)?;
    let repo_name = get_repo_name_from_url(remote_url)?;

    let uri = format!("https://api.github.com/repos/{}/{}/languages", repo_owner, repo_name);

    // let json_data: serde_json::Value = reqwest::Client::new()
    let json_data: serde_json::Value = get_request_wrapper(&uri)
        .await?
        .json()
        .await?;
    
    let lang_array: &serde_json::Map<String, serde_json::Value> = json_data.as_object().unwrap();
    let langs: Vec<String> = lang_array.keys().cloned().collect();

    debug!("Successfully got lanuages");
    return Ok(langs);
}

pub async fn count_recommended_issues(remote_url: &String, recommended_issue_labels: &Vec<String>) -> Result<usize, Box<dyn Error>> {
    debug!("Getting recommended issues");
    let repo_owner = get_repo_owner_from_url(remote_url)?;
    let repo_name = get_repo_name_from_url(remote_url)?;

    let mut ret = 0;

    for label in recommended_issue_labels.iter() {
        // TODO this doesn't handle duplicates
        let uri = format!("https://api.github.com/repos/{}/{}/issues?labels={}", repo_owner, repo_name, label);

        let json_data: Vec<serde_json::Value> = get_request_wrapper(&uri)
            .await?
            .json()
            .await?; 

        ret+=json_data.len();
    }

    debug!("Successfully got number of recommended issues");
    return Ok(ret);
}
