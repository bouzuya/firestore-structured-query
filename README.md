# firestore-structured-query

A Firestore StructuredQuery builder.

[![ci](https://github.com/bouzuya/firestore-structured-query/workflows/ci/badge.svg)](https://github.com/bouzuya/firestore-structured-query/actions)
[![crates.io](https://img.shields.io/crates/v/firestore-structured-query)](https://crates.io/crates/firestore-structured-query)
[![docs.rs](https://img.shields.io/docsrs/firestore-structured-query)](https://docs.rs/firestore-structured-query)
![license](https://img.shields.io/crates/l/firestore-structured-query)

## Examples

```rust
use firestore_structured_query::{FieldPath, Filter, Query};
use google_api_proto::google::firestore::v1::{
    structured_query, value::ValueType, ArrayValue, Cursor, StructuredQuery, Value,
};

let _ = StructuredQuery::from(
    // or Query::collection_group(...)
    Query::collection("collection_id1")
        .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
        .r#where(Filter::and([
            // field filters
            FieldPath::raw("field1").less_than(Value { value_type: Some(ValueType::IntegerValue(1)) })?,
            FieldPath::raw("field2").less_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(2)) })?,
            FieldPath::raw("field3").greater_than(Value { value_type: Some(ValueType::IntegerValue(3)) })?,
            FieldPath::raw("field4").greater_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(4)) })?,
            FieldPath::raw("field5").equal(Value { value_type: Some(ValueType::IntegerValue(5)) })?,
            FieldPath::raw("field6").not_equal(Value { value_type: Some(ValueType::IntegerValue(6)) })?,
            FieldPath::raw("field7").array_contains(Value { value_type: Some(ValueType::IntegerValue(7)) })?,
            FieldPath::raw("field8").r#in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(8)) }] })) })?,
            FieldPath::raw("field9").array_contains_any(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(9)) }] })) })?,
            FieldPath::raw("field10").not_in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(10)) }] })) })?,
            // unary filters
            FieldPath::raw("field11").is_nan()?,
            FieldPath::raw("field12").is_not_nan()?,
            FieldPath::raw("field13").is_not_null()?,
            FieldPath::raw("field14").is_null()?,
            // composite filters
            Filter::and([
                FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
                FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
            ]),
            Filter::or([
                FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
                FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
            ]),
        ]))
        .order_by([
            FieldPath::raw("field1").ascending(),
            FieldPath::raw("field2").descending(),
        ])
        // .start_after(...)
        .start_at([
            Value { value_type: Some(ValueType::IntegerValue(1))},
            Value { value_type: Some(ValueType::IntegerValue(2))},
        ])
        // .end_before(...)
        .end_at([
            Value { value_type: Some(ValueType::IntegerValue(1))},
            Value { value_type: Some(ValueType::IntegerValue(2))},
        ])
        .offset(1_i32)
        .limit(2_i32),
);

// If "serde" feature is enabled.
use firestore_structured_query::to_value;
let _ = StructuredQuery::from(
    // or Query::collection_group(...)
    Query::collection("collection_id1")
        .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
        .r#where(Filter::and([
            // field filters
            FieldPath::raw("field1").less_than(&1)?,
            FieldPath::raw("field2").less_than_or_equal(&2)?,
            FieldPath::raw("field3").greater_than(&3)?,
            FieldPath::raw("field4").greater_than_or_equal(&4)?,
            FieldPath::raw("field5").equal(&5)?,
            FieldPath::raw("field6").not_equal(&6)?,
            FieldPath::raw("field7").array_contains(&7)?,
            FieldPath::raw("field8").r#in(&[8])?,
            FieldPath::raw("field9").array_contains_any(&[9])?,
            FieldPath::raw("field10").not_in(&[10])?,
            // unary filters
            FieldPath::raw("field11").is_nan()?,
            FieldPath::raw("field12").is_not_nan()?,
            FieldPath::raw("field13").is_not_null()?,
            FieldPath::raw("field14").is_null()?,
            // composite filters
            Filter::and([
                FieldPath::raw("f").equal(&"a")?,
                FieldPath::raw("f").equal(&"b")?,
            ]),
            Filter::or([
                FieldPath::raw("f").equal(&"a")?,
                FieldPath::raw("f").equal(&"b")?,
            ]),
        ]))
        .order_by([
            FieldPath::raw("field1").ascending(),
            FieldPath::raw("field2").descending(),
        ])
        // .start_after(...)
        .start_at([
            to_value(&1)?,
            to_value(&2)?
        ])
        // .end_before(...)
        .end_at([
            to_value(&1)?,
            to_value(&2)?,
        ])
        .offset(1_i32)
        .limit(2_i32),
);
```

## Version matrices

| firestore-structured-query | [google-api-proto]  | tonic     |
|----------------------------|---------------------|-----------|
| <0.7.0                     | (unknown)           | (unknown) |
| >=0.7.0, <0.9.0            | >=1.516.0, <1.557.0 | 0.11.x    |
| >=0.9.0                    | >=1.557.0           | 0.11.x    |

[google-api-proto]: https://github.com/mechiru/google-api-proto
