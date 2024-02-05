use google_api_proto::google::firestore::v1::structured_query;

use crate::FieldPath;

#[derive(Clone, Debug, PartialEq)]
pub struct Order(structured_query::Order);

impl Order {
    pub(crate) fn new(field_path: FieldPath, direction: structured_query::Direction) -> Self {
        Self(structured_query::Order {
            field: Some(structured_query::FieldReference::from(field_path)),
            direction: direction as i32,
        })
    }
}

impl std::convert::From<Order> for structured_query::Order {
    fn from(order: Order) -> Self {
        order.0
    }
}
