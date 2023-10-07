use std::error::Error;
use url::Url;

fn get_path_segments_from_url(remote_url: &str) -> Result<Vec<&'static str>, Box<dyn Error>> {
    let url = Url::parse(&remote_url)?;
    let path_segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();

    // https://github.com/{REPO_OWNER}/{REPO_NAME}
    assert_eq!(2, path_segments.len());

    return Ok(path_segments.clone()); 

}

fn get_repo_owner_from_url(remote_url: &str) -> Result<&str, Box<dyn Error>> {
    let path_segments = get_path_segments_from_url(remote_url)?;
    return Ok(path_segments[0]);
}

fn get_repo_name_from_url(remote_url: &str) -> Result<&str, Box<dyn Error>> {
    let path_segments = get_path_segments_from_url(remote_url)?;
    return Ok(path_segments[1]);
}

pub async fn check_remote(remote_url: String) -> Result<(), Box<dyn Error>> {
    let repo_owner = get_repo_owner_from_url(&remote_url)?;
    let repo_name = get_repo_name_from_url(&remote_url)?;

    // TODO finish checking remote

    return Ok(());
}