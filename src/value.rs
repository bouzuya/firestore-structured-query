use crate::Result;

pub fn to_value<T>(v: &T) -> Result<google_api_proto::google::firestore::v1::Value>
where
    T: serde::Serialize,
{
    Ok(serde_firestore_value::to_value(v)?)
}

pub trait IntoValue {
    fn into_value(self) -> Result<google_api_proto::google::firestore::v1::Value>;
}

impl IntoValue for google_api_proto::google::firestore::v1::Value {
    fn into_value(self) -> Result<google_api_proto::google::firestore::v1::Value> {
        Ok(self)
    }
}

impl<T: serde::Serialize> IntoValue for &T {
    fn into_value(self) -> Result<google_api_proto::google::firestore::v1::Value> {
        Ok(serde_firestore_value::to_value(self)?)
    }
}
