#![allow(missing_docs)]

// Deprecated: Replace google-api-proto with googleapis-tonic-google-firestore-v1.
// #[test]
// fn test_impl_from_field_path_for_field_reference() -> firestore_structured_query::Result<()> {
//     // Added: impl From<FieldPath> for FieldReference
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::structured_query;
//     let field_path1 = FieldPath::raw("field1");
//     assert_eq!(
//         structured_query::FieldReference::from(field_path1),
//         structured_query::FieldReference {
//             field_path: "field1".to_string(),
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_collection() -> firestore_structured_query::Result<()> {
//     // Added: Query::collection
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
//     let query1 = Query::collection("collection_id1");
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: false,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_collection_group() -> firestore_structured_query::Result<()> {
//     // Added: Query::collection_group
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
//     let query1 = Query::collection_group("collection_id1");
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_end_at() -> firestore_structured_query::Result<()> {
//     // Added: Query::end_at
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, Cursor, StructuredQuery, Value,
//     };
//     let query1 = Query::collection_group("collection_id1").end_at([
//         Value {
//             value_type: Some(ValueType::IntegerValue(1)),
//         },
//         Value {
//             value_type: Some(ValueType::IntegerValue(2)),
//         },
//     ]);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: None,
//             end_at: Some(Cursor {
//                 values: vec![
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(1)),
//                     },
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(2)),
//                     },
//                 ],
//                 before: false,
//             }),
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_end_before() -> firestore_structured_query::Result<()> {
//     // Added: Query::end_before
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, Cursor, StructuredQuery, Value,
//     };
//     let query1 = Query::collection_group("collection_id1").end_before([
//         Value {
//             value_type: Some(ValueType::IntegerValue(1)),
//         },
//         Value {
//             value_type: Some(ValueType::IntegerValue(2)),
//         },
//     ]);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: None,
//             end_at: Some(Cursor {
//                 values: vec![
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(1)),
//                     },
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(2)),
//                     },
//                 ],
//                 before: true,
//             }),
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_limit() -> firestore_structured_query::Result<()> {
//     // Added: Query::limit
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
//     let query1 = Query::collection_group("collection_id1").limit(1_i32);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: Some(1_i32),
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_offset() -> firestore_structured_query::Result<()> {
//     // Added: Query::offset
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
//     let query1 = Query::collection_group("collection_id1").offset(1_i32);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: None,
//             end_at: None,
//             offset: 1_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_order_by() -> firestore_structured_query::Result<()> {
//     // Added: Query::order_by
//     use firestore_structured_query::{FieldPath, Query};
//     use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
//     let query1 = Query::collection_group("collection_id1").order_by([
//         FieldPath::raw("field1").ascending(),
//         FieldPath::raw("field2").descending(),
//     ]);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![
//                 structured_query::Order::from(FieldPath::raw("field1").ascending()),
//                 structured_query::Order::from(FieldPath::raw("field2").descending()),
//             ],
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     let order_by1 = vec![
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field1".to_string(),
//             }),
//             direction: structured_query::Direction::Ascending as i32,
//         },
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field2".to_string(),
//             }),
//             direction: structured_query::Direction::Descending as i32,
//         },
//     ];
//     let query2 = Query::collection_group("collection_id1").order_by(order_by1.clone());
//     assert_eq!(
//         StructuredQuery::from(query2),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: order_by1,
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_select() -> firestore_structured_query::Result<()> {
//     // Added: Query::select
//     use firestore_structured_query::{FieldPath, Query};
//     use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
//     let query1 = Query::collection_group("collection_id1")
//         .select([FieldPath::raw("field1"), FieldPath::raw("field2")]);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: Some(structured_query::Projection {
//                 fields: vec![
//                     structured_query::FieldReference {
//                         field_path: "field1".to_string(),
//                     },
//                     structured_query::FieldReference {
//                         field_path: "field2".to_string(),
//                     },
//                 ],
//             }),
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_start_after() -> firestore_structured_query::Result<()> {
//     // Added: Query::start_after
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, Cursor, StructuredQuery, Value,
//     };
//     let query1 = Query::collection_group("collection_id1").start_after([
//         Value {
//             value_type: Some(ValueType::IntegerValue(1)),
//         },
//         Value {
//             value_type: Some(ValueType::IntegerValue(2)),
//         },
//     ]);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: Some(Cursor {
//                 values: vec![
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(1)),
//                     },
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(2)),
//                     },
//                 ],
//                 before: false,
//             }),
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_start_at() -> firestore_structured_query::Result<()> {
//     // Added: Query::start_at
//     use firestore_structured_query::Query;
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, Cursor, StructuredQuery, Value,
//     };
//     let query1 = Query::collection_group("collection_id1").start_at([
//         Value {
//             value_type: Some(ValueType::IntegerValue(1)),
//         },
//         Value {
//             value_type: Some(ValueType::IntegerValue(2)),
//         },
//     ]);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: None,
//             order_by: vec![],
//             start_at: Some(Cursor {
//                 values: vec![
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(1)),
//                     },
//                     Value {
//                         value_type: Some(ValueType::IntegerValue(2)),
//                     },
//                 ],
//                 before: true,
//             }),
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }

// Deprecated: provides when `serde` feature is enabled
// #[test]
// fn test_to_value() -> firestore_structured_query::Result<()> {
//     // Added: ::to_value
//     use firestore_structured_query::to_value;
//     use google_api_proto::google::firestore::v1::{value::ValueType, Value};
//     assert_eq!(
//         to_value(&1_i64)?,
//         Value {
//             value_type: Some(ValueType::IntegerValue(1_i64)),
//         }
//     );
//     Ok(())
// }

// Deprecated: Changed in v0.9.0. The output has changed because StructuredQuery::find_nearest has been added in google_api_proto v1.557.0.
// #[test]
// fn test_query_where() -> firestore_structured_query::Result<()> {
//     // Added: Query::r#where
//     use firestore_structured_query::{FieldPath, Query};
//     use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
//     let query1 =
//         Query::collection_group("collection_id1").r#where(FieldPath::raw("field1").is_nan()?);
//     assert_eq!(
//         StructuredQuery::from(query1),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: Some(structured_query::Filter {
//                 filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
//                     structured_query::UnaryFilter {
//                         op: structured_query::unary_filter::Operator::IsNan as i32,
//                         operand_type: Some(structured_query::unary_filter::OperandType::Field(
//                             structured_query::FieldReference {
//                                 field_path: "field1".to_string(),
//                             }
//                         ))
//                     }
//                 ))
//             }),
//             order_by: vec![],
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     let filter1 = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
//             structured_query::UnaryFilter {
//                 op: structured_query::unary_filter::Operator::IsNan as i32,
//                 operand_type: Some(structured_query::unary_filter::OperandType::Field(
//                     structured_query::FieldReference {
//                         field_path: "field1".to_string(),
//                     },
//                 )),
//             },
//         )),
//     };
//     let query2 = Query::collection_group("collection_id1").r#where(filter1.clone());
//     assert_eq!(
//         StructuredQuery::from(query2),
//         StructuredQuery {
//             select: None,
//             from: vec![structured_query::CollectionSelector {
//                 collection_id: "collection_id1".to_string(),
//                 all_descendants: true,
//             }],
//             r#where: Some(filter1),
//             order_by: vec![],
//             start_at: None,
//             end_at: None,
//             offset: 0_i32,
//             limit: None,
//         }
//     );
//     Ok(())
// }
