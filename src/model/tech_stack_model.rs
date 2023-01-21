use super::serialize_object_id;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TechStack {
    #[serde(
        rename(deserialize = "_id", serialize = "id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechStackUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}
