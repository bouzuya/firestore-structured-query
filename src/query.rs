use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};

pub struct Query(StructuredQuery);

impl Query {
    pub fn collection<S>(collection_id: S) -> Self
    where
        S: Into<String>,
    {
        Self(StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: collection_id.into(),
                all_descendants: false,
            }],
            r#where: None,
            order_by: vec![],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
        })
    }

    pub fn collection_group<S>(collection_id: S) -> Self
    where
        S: Into<String>,
    {
        Self(StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: collection_id.into(),
                all_descendants: true,
            }],
            r#where: None,
            order_by: vec![],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
        })
    }

    pub fn order_by<I>(mut self, order_by: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<structured_query::Order>,
    {
        self.0.order_by = order_by.into_iter().map(Into::into).collect();
        self
    }

    pub fn r#where<F>(mut self, filter: F) -> Self
    where
        F: Into<structured_query::Filter>,
    {
        self.0.r#where = Some(filter.into());
        self
    }
}

impl std::convert::From<Query> for StructuredQuery {
    fn from(query: Query) -> Self {
        query.0
    }
}
