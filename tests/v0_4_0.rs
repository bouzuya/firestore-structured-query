#[test]
fn test_query_collection() -> firestore_structured_query::Result<()> {
    // Added: Query::collection
    use firestore_structured_query::Query;
    use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    let query1 = Query::collection("collection_id1");
    assert_eq!(
        StructuredQuery::from(query1),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: false,
            }],
            r#where: None,
            order_by: vec![],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_collection_group() -> firestore_structured_query::Result<()> {
    // Added: Query::collection_group
    use firestore_structured_query::Query;
    use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    let query1 = Query::collection_group("collection_id1");
    assert_eq!(
        StructuredQuery::from(query1),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: true,
            }],
            r#where: None,
            order_by: vec![],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
        }
    );
    Ok(())
}
