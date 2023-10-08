use std::error::Error;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};
use std::str;
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FernFile {
    name: String,
    description: String,
    technologies: Vec<String>,
    difficulty: u8,
    recommended_issue_labels: Vec<String>,
}

fn read_raw_b64(content: String) -> Result<String, Box<dyn Error>> {
    println!("{}", content);
    let decoded_bytes = general_purpose::STANDARD.decode(content).unwrap().clone();
    return Ok(String::from_utf8(decoded_bytes)?);
}

pub fn read_b64_content(content: String) -> Result<FernFile, Box<dyn Error>> {
    let json_str = read_raw_b64(content)?;
    let formatted_json: FernFile = serde_json::from_str(&json_str)?;
    return Ok(formatted_json);
}