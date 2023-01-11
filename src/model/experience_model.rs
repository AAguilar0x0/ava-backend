use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub role: String,
    pub company: String,
    pub description: String,
    pub start: DateTime,
    pub end: DateTime,
    pub tech_stack: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperienceUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_stack: Option<Vec<String>>,
}
