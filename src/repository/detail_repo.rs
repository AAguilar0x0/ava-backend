use std::env;
extern crate dotenv;

use actix_web::http::StatusCode;
use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::ErrorKind,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::model::detail_model::Detail;

pub struct DetailRepo {
    col: Collection<Detail>,
}

impl DetailRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").map_err(|err| err.to_string()).unwrap();
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("ava");
        let col: Collection<Detail> = db.collection("Detail");
        DetailRepo { col }
    }

    pub async fn create_record(
        &self,
        new_record: Detail,
    ) -> Result<InsertOneResult, (StatusCode, String)> {
        let record = self.col.insert_one(new_record, None).await.map_err(|err| {
            (
                match *err.kind {
                    ErrorKind::InvalidArgument { .. } => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                err.to_string(),
            )
        })?;

        Ok(record)
    }

    pub async fn get_record(&self, id: &String) -> Result<Detail, (StatusCode, String)> {
        let obj_id = ObjectId::parse_str(id)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID".to_owned()))?;
        let filter = doc! {"_id": obj_id};
        let record = self.col.find_one(filter, None).await.map_err(|err| {
            (
                match *err.kind {
                    ErrorKind::InvalidArgument { .. } => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                err.to_string(),
            )
        })?;

        Ok(record.ok_or((StatusCode::NOT_FOUND, "ID not found!".to_owned()))?)
    }

    pub async fn update_record(
        &self,
        id: &String,
        new_record: Detail,
    ) -> Result<UpdateResult, (StatusCode, String)> {
        let obj_id =
            ObjectId::parse_str(id).map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_record.id,
                    "name": new_record.name,
                    "description": new_record.description,
                    "image": new_record.image,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .map_err(|err| {
                (
                    match *err.kind {
                        ErrorKind::InvalidArgument { .. } => StatusCode::BAD_REQUEST,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    },
                    err.to_string(),
                )
            })?;
        Ok(updated_doc)
    }

    pub async fn delete_record(&self, id: &String) -> Result<DeleteResult, (StatusCode, String)> {
        let obj_id =
            ObjectId::parse_str(id).map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
        let filter = doc! {"_id": obj_id};
        let record = self.col.delete_one(filter, None).await.map_err(|err| {
            (
                match *err.kind {
                    // ErrorKind::InvalidArgument { message } => todo!(),
                    // ErrorKind::Authentication { message } => todo!(),
                    // ErrorKind::BsonDeserialization(_) => todo!(),
                    // ErrorKind::BsonSerialization(_) => todo!(),
                    // ErrorKind::BulkWrite(_) => todo!(),
                    // ErrorKind::Command(_) => todo!(),
                    // ErrorKind::DnsResolve { message } => todo!(),
                    // ErrorKind::Internal { message } => todo!(),
                    // ErrorKind::Io(_) => todo!(),
                    // ErrorKind::ConnectionPoolCleared { message } => todo!(),
                    // ErrorKind::InvalidResponse { message } => todo!(),
                    // ErrorKind::ServerSelection { message } => todo!(),
                    // ErrorKind::SessionsNotSupported => todo!(),
                    // ErrorKind::InvalidTlsConfig { message } => todo!(),
                    // ErrorKind::Write(_) => todo!(),
                    // ErrorKind::Transaction { message } => todo!(),
                    // ErrorKind::IncompatibleServer { message } => todo!(),
                    // ErrorKind::MissingResumeToken => todo!(),
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                err.to_string(),
            )
        })?;

        Ok(record)
    }

    pub async fn get_all_record(&self) -> Result<Vec<Detail>, (StatusCode, String)> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        let mut records: Vec<Detail> = Vec::new();
        while let Some(record) = cursors
            .try_next()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        {
            records.push(record)
        }
        Ok(records)
    }
}
