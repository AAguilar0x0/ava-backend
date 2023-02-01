use mongodb::bson::oid::ObjectId;
use serde::Serializer;

pub mod detail_model;
pub mod experience_model;
pub mod project_model;
pub mod tech_stack_model;
pub mod user_model;

fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
        Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
        None => serializer.serialize_none(),
    }
}
