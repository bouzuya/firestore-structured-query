use google_api_proto::google::firestore::v1::structured_query;

use crate::FieldPath;

pub trait FieldPathOrderExt {
    fn ascending(&self) -> structured_query::Order;
    fn descending(&self) -> structured_query::Order;
}

impl FieldPathOrderExt for FieldPath {
    fn ascending(&self) -> structured_query::Order {
        order(self.clone(), structured_query::Direction::Ascending)
    }

    fn descending(&self) -> structured_query::Order {
        order(self.clone(), structured_query::Direction::Descending)
    }
}

fn order(field_path: FieldPath, direction: structured_query::Direction) -> structured_query::Order {
    structured_query::Order {
        field: Some(structured_query::FieldReference::from(field_path)),
        direction: direction as i32,
    }
}
