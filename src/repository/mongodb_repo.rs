use std::env;
extern crate dotenv;

use actix_web::http::StatusCode;
use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::ErrorKind,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct MongoDB<T> {
    col: Collection<T>,
    name: String,
}

impl<T> MongoDB<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    pub async fn init(collection: &str) -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").map_err(|err| err.to_string()).unwrap();
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("ava");
        let col: Collection<T> = db.collection(collection);
        MongoDB {
            col,
            name: collection.to_owned(),
        }
    }

    pub async fn create_record(
        &self,
        new_record: T,
    ) -> Result<InsertOneResult, (StatusCode, String)> {
        let record = self.col.insert_one(new_record, None).await.map_err(|err| {
            (
                match *err.kind {
                    ErrorKind::InvalidArgument { .. } => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                format!("{} MongoDB Repo Error: {}", self.name, err.to_string()),
            )
        })?;

        Ok(record)
    }

    pub async fn get_all_record(&self) -> Result<Vec<T>, (StatusCode, String)> {
        let mut cursors = self.col.find(None, None).await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{} MongoDB Repo Error: {}", self.name, err.to_string()),
            )
        })?;
        let mut records = Vec::new();
        while let Some(record) = cursors.try_next().await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{} MongoDB Repo Error: {}", self.name, err.to_string()),
            )
        })? {
            records.push(record)
        }
        Ok(records)
    }

    pub async fn get_record(&self, id: &str) -> Result<T, (StatusCode, String)> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                format!("{} MongoDB Repo Error: Invalid ID", self.name),
            )
        })?;
        let filter = doc! {"_id": obj_id};
        let record = self.col.find_one(filter, None).await.map_err(|err| {
            (
                match *err.kind {
                    ErrorKind::InvalidArgument { .. } => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                format!("{} MongoDB Repo Error: {}", self.name, err.to_string()),
            )
        })?;

        Ok(record.ok_or((
            StatusCode::NOT_FOUND,
            format!("{} MongoDB Repo Error: ID not found", self.name),
        ))?)
    }

    pub async fn update_record(
        &self,
        filter: Document,
        new_record: Document,
    ) -> Result<UpdateResult, (StatusCode, String)> {
        let updated_doc = self
            .col
            .update_one(filter, new_record, None)
            .await
            .map_err(|err| {
                (
                    match *err.kind {
                        ErrorKind::InvalidArgument { .. } => StatusCode::BAD_REQUEST,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    },
                    format!("{} MongoDB Repo Error: {}", self.name, err.to_string()),
                )
            })?;
        Ok(updated_doc)
    }

    pub async fn delete_record(&self, id: &str) -> Result<DeleteResult, (StatusCode, String)> {
        let obj_id = ObjectId::parse_str(id).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                format!("{} MongoDB Repo Error: Invalid ID", self.name),
            )
        })?;
        let filter = doc! {"_id": obj_id};
        let record = self.col.delete_one(filter, None).await.map_err(|err| {
            (
                match *err.kind {
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                format!("{} MongoDB Repo Error: {}", self.name, err.to_string()),
            )
        })?;

        Ok(record)
    }
}
