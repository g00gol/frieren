use mongodb::{Client, Database, Cursor, Collection, options::{ClientOptions, ResolverConfig}, options::FindOneAndReplaceOptions, bson::oid::ObjectId};
use bson::doc;
use chrono::{DateTime, Utc};
use std::env;
use std::error::Error;
use serde::{Serialize, Deserialize};
use log::{debug};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repo {
    pub _id: ObjectId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub languages: Option<Vec<String>>,
    pub technologies: Option<Vec<String>>,
    pub difficulty: Option<u32>, // 0-4
    pub recommended_issue_labels: Option<Vec<String>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_updated: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date_created: DateTime<Utc>,
    pub stars: Option<u64>,
    pub recommended_issues_count: Option<usize>,
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
    debug!("Attempting to query DB");
    let collection = get_repos_collection().await?;
    let cursor = collection.find(None, None).await?;
    
    debug!("Successully queried DB");
    return Ok(cursor);
}

pub async fn update_repo(new_repo: &Repo) -> Result<(), Box<dyn Error>> {
    debug!("Attempting to update DB");
    let col = get_repos_collection().await?;
    col.find_one_and_replace(
        doc!{
            "_id": new_repo._id
        },
        new_repo,
        FindOneAndReplaceOptions::builder().build()
    ).await?;
    debug!("Successfully updated DB");
    return Ok(());
}

pub async fn delete_entry(_id: &ObjectId) -> Result<(), Box<dyn Error>> {
    debug!("Attempting to delete entry {:?}", _id);
    let col = get_repos_collection().await?;
    col.find_one_and_delete(doc!{
        "_id": _id
    }, None, ).await?;
    return Ok(());
}