use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub role: String,
    pub company: String,
    pub description: String,
    pub start: u32,
    pub end: u32,
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
    pub start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_stack: Option<Vec<String>>,
}
