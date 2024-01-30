use google_api_proto::google::firestore::v1::structured_query;

use crate::FieldPath;

pub trait FieldPathOrderExt {
    fn ascending(&self) -> structured_query::Order;
    fn descending(&self) -> structured_query::Order;
}

impl FieldPathOrderExt for FieldPath {
    fn ascending(&self) -> structured_query::Order {
        structured_query::Order {
            field: Some(self.to_field_reference()),
            direction: structured_query::Direction::Ascending as i32,
        }
    }

    fn descending(&self) -> structured_query::Order {
        structured_query::Order {
            field: Some(self.to_field_reference()),
            direction: structured_query::Direction::Descending as i32,
        }
    }
}
