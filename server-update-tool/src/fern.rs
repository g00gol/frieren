// use std::error::Error;
// use base64::{Engine as _, engine::general_purpose};

// pub struct FernFile {
//     name: String,
//     description: String,
//     technologies: Vec<String>,
//     difficulty: u8,
//     recommended_issue_labels: Vec<String>,
// }

// fn read_raw_b64(content: String) -> Result<String, Box<dyn Error>> {
//     let decoded_bytes = general_purpose::STANDARD_NO_PAD.decode(content).unwrap();
//     return String::from_utf8_lossless(&decoded_bytes);
// }

// pub fn read_b64_content(content: String) -> Result<FernFile, Box<dyn Error>> {
//     let yaml_str = read_raw_b64(content)?;
//     println!("{}", yaml_str);
// }