use google_api_proto::google::firestore::v1::structured_query;

use crate::FieldPath;

/// A Firestore query order.
///
/// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#order>
///
/// ```rust
/// # fn example_order() -> firestore_structured_query::Result<()> {
/// use firestore_structured_query::{FieldPath, Order};
/// use google_api_proto::google::firestore::v1::structured_query;
/// let order1: Order = FieldPath::raw("field1").ascending();
/// let order2: Order = FieldPath::raw("field2").descending();
/// assert_eq!(
///     structured_query::Order::from(order1),
///     structured_query::Order {
///         field: Some(structured_query::FieldReference {
///             field_path: "field1".to_string()
///         }),
///         direction: structured_query::Direction::Ascending as i32
///     }
/// );
/// assert_eq!(
///     structured_query::Order::from(order2),
///     structured_query::Order {
///         field: Some(structured_query::FieldReference {
///             field_path: "field2".to_string()
///         }),
///         direction: structured_query::Direction::Descending as i32
///     }
/// );
/// #     Ok(())
/// # }
/// ```
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
