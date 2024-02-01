#[test]
fn test_simple_example() -> firestore_structured_query::Result<()> {
    use firestore_structured_query::{
        FieldPath, FieldPathFilterExt as _, FieldPathOrderExt as _, Filter, Query,
    };
    use google_api_proto::google::firestore::v1::{
        structured_query, value::ValueType, StructuredQuery, Value,
    };

    let _ = StructuredQuery {
        select: Some(structured_query::Projection {
            fields: vec![
                FieldPath::raw("field1").into(),
                FieldPath::raw("field2").into(),
            ],
        }),
        from: vec![],
        r#where: Some(
            Filter::and([
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
            ])
            .into(),
        ),
        order_by: vec![
            FieldPath::raw("field1").ascending(),
            FieldPath::raw("field2").descending(),
        ],
        start_at: None,
        end_at: None,
        offset: 0_i32,
        limit: None,
    };

    let _ = StructuredQuery::from(
        // or Query::collection_group(...)
        Query::collection("collection_id1")
            .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
            .r#where(FieldPath::raw("field1").less_than(&1)?)
            .order_by([
                FieldPath::raw("field1").ascending(),
                FieldPath::raw("field2").descending(),
            ])
            // .start_after(...)
            .start_at([
                Value {
                    value_type: Some(ValueType::IntegerValue(1)),
                },
                Value {
                    value_type: Some(ValueType::IntegerValue(2)),
                },
            ]),
    );

    Ok(())
}
