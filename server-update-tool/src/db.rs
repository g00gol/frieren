use mongodb::{Client, Database, Cursor, Collection, options::{ClientOptions, ResolverConfig}};
use mongodb::bson::{DateTime, to_document};
use std::env;
use std::error::Error;
use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repo {
    pub _id: ObjectId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub languages: Option<Vec<String>>,
    pub technologies: Option<Vec<String>>,
    pub difficulty: Option<u32>, // 0-4
    pub recommended_issue_labels: Option<Vec<String>>,
    pub last_updated: Option<DateTime>,
    pub stars: Option<u32>,
    pub recommended_issues_count: Option<u32>,
    pub repo_origin: String,
    pub fern_branch: Option<String>,
    pub hash: Option<String>
}

async fn get_mongo_client() -> Result<Client, Box<dyn Error>> {

    let client_uri = env::var("MONGO_URI").expect("No MONGO_URI enviroment var!");

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    let client: Client = Client::with_options(options)?;

    return Ok(client);
}

async fn get_mongo_database() -> Result<Database, Box<dyn Error>> {
    let client = get_mongo_client().await?;
    let database_name = env::var("MONGO_DB_NAME").expect("No MONGO_DB_NAME environment var!");
    return Ok(client.database(database_name.as_str()));
}

async fn get_repos_collection() -> Result<Collection<Repo>, Box<dyn Error>> {
    let database = get_mongo_database().await?;
    return Ok(database.collection::<Repo>("repos"));
}

pub async fn get_repos() -> Result<Cursor<Repo>, Box<dyn Error>> {
    let collection = get_repos_collection().await?;
    let cursor = collection.find(None, None).await?;
    
    return Ok(cursor);
}
