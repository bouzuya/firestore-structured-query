// Fixed: version: 0.8.2
// #[test]
// fn test_field_path_new() {
//     // Added: FieldPath::new
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::structured_query;
//     let field_path1 = FieldPath::new(["field1"]);
//     assert_eq!(
//         structured_query::FieldReference::from(field_path1),
//         structured_query::FieldReference {
//             field_path: "field1".to_string(),
//         }
//     );
//     let field_path2 = FieldPath::new(["field1", "field2"]);
//     assert_eq!(
//         structured_query::FieldReference::from(field_path2),
//         structured_query::FieldReference {
//             field_path: "field1.field2".to_string(),
//         }
//     );
//     let field_path3 = FieldPath::new(["foo", "x&y"]);
//     assert_eq!(
//         structured_query::FieldReference::from(field_path3),
//         structured_query::FieldReference {
//             field_path: "foo.`x&y`".to_string(),
//         }
//     );
//     let field_path4 = FieldPath::new(["a`b", r#"a\b"#]);
//     assert_eq!(
//         structured_query::FieldReference::from(field_path4),
//         structured_query::FieldReference {
//             field_path: r#"`a\`b`.`a\b`"#.to_string(),
//         }
//     );
// }
