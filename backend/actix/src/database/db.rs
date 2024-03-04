use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub uuid: String,
    pub task: String,
    pub marked: String
}

pub struct DB {
    pub collection: Collection<Task>
}

impl DB {
    pub async fn new() -> DB {
        let conn_str: String = std::env::var("URI_CONN_STR").expect("Environment variable was not found");
        let client = match Client::with_uri_str(conn_str).await {
            Ok(x) => x,
            Err(_) => panic!("Failed to connect to Database")
        };

        let db = client.database("todo");
        let collect: Collection<Task> = db.collection("tasks");
        DB {collection: collect}
    }

    pub async fn add_task(&self, task: Task) -> Result<String, String> {
        let result = self.collection.insert_one(task, None).await;
        match result {
            Ok(_) => Ok("Inserted".to_string()),
            Err(_) => Err("Failed to add Task".to_string())
        }
    }

    pub async fn get_task(&self) -> Result<Vec<Task>, String> {
        let result = self.collection.find(None, None).await.unwrap().try_collect().await;
        match result {
            Ok(x) => Ok(x),
            Err(_) => Err("Failed to get Task".to_string())
        }
    }
    
    pub async fn update_task(&self, uuid: String) -> Result<String, String> {
        let result = self.collection.update_one(doc! {"uuid" : uuid}, doc! { "$set" : {"marked" : "line-through"}}, None).await;
        match result {
             Ok(_) => Ok("Marked".to_string()),
             Err(_) => Err("Failed to update Task".to_string())           
        }
    }

    pub async fn delete_task(&self, uuid: String) -> Result<String, String> {
        let result = self.collection.delete_one(doc! {"uuid" : uuid}, None).await;
        match result {
            Ok(_) => Ok("Deleted".to_string()),
            Err(_) => Err("This task doesn't exist".to_string())
        }
    }
}
