//! A Firestore StructuredQuery builder
//!
//! # Examples
//!
//! ```rust
//! # fn example_mod_doc() -> firestore_structured_query::Result<()> {
//! use firestore_structured_query::{FieldPath, Filter, Query};
//! use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
//!     structured_query, value::ValueType, ArrayValue, Cursor, StructuredQuery, Value,
//! };
//!
//! let _ = StructuredQuery::from(
//!     // or Query::collection_group(...)
//!     Query::collection("collection_id1")
//!         .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
//!         .r#where(Filter::and([
//!             // field filters
//!             FieldPath::raw("field1").less_than(Value { value_type: Some(ValueType::IntegerValue(1)) })?,
//!             FieldPath::raw("field2").less_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(2)) })?,
//!             FieldPath::raw("field3").greater_than(Value { value_type: Some(ValueType::IntegerValue(3)) })?,
//!             FieldPath::raw("field4").greater_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(4)) })?,
//!             FieldPath::raw("field5").equal(Value { value_type: Some(ValueType::IntegerValue(5)) })?,
//!             FieldPath::raw("field6").not_equal(Value { value_type: Some(ValueType::IntegerValue(6)) })?,
//!             FieldPath::raw("field7").array_contains(Value { value_type: Some(ValueType::IntegerValue(7)) })?,
//!             FieldPath::raw("field8").r#in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(8)) }] })) })?,
//!             FieldPath::raw("field9").array_contains_any(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(9)) }] })) })?,
//!             FieldPath::raw("field10").not_in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(10)) }] })) })?,
//!             // unary filters
//!             FieldPath::raw("field11").is_nan()?,
//!             FieldPath::raw("field12").is_not_nan()?,
//!             FieldPath::raw("field13").is_not_null()?,
//!             FieldPath::raw("field14").is_null()?,
//!             // composite filters
//!             Filter::and([
//!                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
//!                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
//!             ]),
//!             Filter::or([
//!                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
//!                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
//!             ]),
//!         ]))
//!         .order_by([
//!             FieldPath::raw("field1").ascending(),
//!             FieldPath::raw("field2").descending(),
//!         ])
//!         // .start_after(...)
//!         .start_at([
//!             Value { value_type: Some(ValueType::IntegerValue(1))},
//!             Value { value_type: Some(ValueType::IntegerValue(2))},
//!         ])
//!         // .end_before(...)
//!         .end_at([
//!             Value { value_type: Some(ValueType::IntegerValue(1))},
//!             Value { value_type: Some(ValueType::IntegerValue(2))},
//!         ])
//!         .offset(1_i32)
//!         .limit(2_i32),
//! );
//! #     Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! Name | Description | Default?
//! ---|---|---
//! `serde` | Enable support for `serde::Serialize` using the `serde_serialize_value` crate. | No
//!
mod error;
mod field_path;
mod filter;
mod order;
mod query;
mod value;

pub use self::error::{Error, Result};
pub use self::field_path::FieldPath;
pub use self::filter::Filter;
pub use self::order::Order;
pub use self::query::Query;
#[cfg(feature = "serde")]
pub use self::value::to_value;
pub use self::value::IntoValue;
