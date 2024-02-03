// Deprected: Removed FieldPathOrderExt in v0.6.0
// #[test]
// fn test_field_path_filter_ext_ascending() -> firestore_structured_query::Result<()> {
//     // Added: FieldPathOrderExt::ascending
//     use firestore_structured_query::{FieldPath, FieldPathOrderExt as _};
//     use google_api_proto::google::firestore::v1::structured_query;
//     assert_eq!(
//         FieldPath::raw("field1").ascending(),
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field1".to_string()
//             }),
//             direction: structured_query::Direction::Ascending as i32
//         }
//     );
//     Ok(())
// }

// Deprected: Removed FieldPathOrderExt in v0.6.0
// #[test]
// fn test_field_path_filter_ext_descending() -> firestore_structured_query::Result<()> {
//     // Added: FieldPathOrderExt::descending
//     use firestore_structured_query::{FieldPath, FieldPathOrderExt as _};
//     use google_api_proto::google::firestore::v1::structured_query;
//     assert_eq!(
//         FieldPath::raw("field1").descending(),
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field1".to_string()
//             }),
//             direction: structured_query::Direction::Descending as i32
//         }
//     );
//     Ok(())
// }
