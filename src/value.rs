use crate::Result;

#[cfg(feature = "serde")]
pub fn to_value<T>(v: &T) -> Result<google_api_proto::google::firestore::v1::Value>
where
    T: serde::Serialize,
{
    Ok(serde_firestore_value::to_value(v)
        .map_err(Box::<dyn std::error::Error + Send + Sync>::from)?)
}

/// A Firestore value converter trait.
pub trait IntoValue {
    /// Convert the value into a Firestore value.
    fn into_value(self) -> Result<google_api_proto::google::firestore::v1::Value>;
}

impl IntoValue for google_api_proto::google::firestore::v1::Value {
    fn into_value(self) -> Result<google_api_proto::google::firestore::v1::Value> {
        Ok(self)
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> IntoValue for &T {
    fn into_value(self) -> Result<google_api_proto::google::firestore::v1::Value> {
        Ok(serde_firestore_value::to_value(self)
            .map_err(Box::<dyn std::error::Error + Send + Sync>::from)?)
    }
}
