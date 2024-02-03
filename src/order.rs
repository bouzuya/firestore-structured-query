use google_api_proto::google::firestore::v1::structured_query;

use crate::FieldPath;

pub(crate) fn order(
    field_path: FieldPath,
    direction: structured_query::Direction,
) -> structured_query::Order {
    structured_query::Order {
        field: Some(structured_query::FieldReference::from(field_path)),
        direction: direction as i32,
    }
}
