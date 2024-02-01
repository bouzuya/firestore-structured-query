use crate::Result;

pub fn to_value<T>(v: &T) -> Result<google_api_proto::google::firestore::v1::Value>
where
    T: serde::Serialize,
{
    Ok(serde_firestore_value::to_value(v)?)
}
