# firestore-structured-query

A Firestore StructuredQuery builder.

[![ci](https://github.com/bouzuya/firestore-structured-query/workflows/ci/badge.svg)](https://github.com/bouzuya/firestore-structured-query/actions)
[![crates.io](https://img.shields.io/crates/v/firestore-structured-query)](https://crates.io/crates/firestore-structured-query)
[![docs.rs](https://img.shields.io/docsrs/firestore-structured-query)](https://docs.rs/firestore-structured-query)
[![license](https://img.shields.io/crates/l/firestore-structured-query)](#license)

## Examples

```rust
#[test]
fn test_simple_example() -> firestore_structured_query::Result<()> {
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _, Filter};
    use google_api_proto::google::firestore::v1::StructuredQuery;

    let _ = StructuredQuery {
        select: None,
        from: vec![],
        r#where: Some(
            Filter::and([
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
                Filter::or([
                    FieldPath::raw("f").equal(&"a")?,
                    FieldPath::raw("f").equal(&"b")?,
                ]),
            ])
            .into(),
        ),
        order_by: vec![],
        start_at: None,
        end_at: None,
        offset: 0_i32,
        limit: None,
    };
    Ok(())
}
```
