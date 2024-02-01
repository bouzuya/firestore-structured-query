use google_api_proto::google::firestore::v1::{structured_query, Cursor, StructuredQuery};

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

    pub fn end_at<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = google_api_proto::google::firestore::v1::Value>,
    {
        self.0.end_at = Some(Cursor {
            values: values.into_iter().collect(),
            before: false,
        });
        self
    }

    pub fn end_before<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = google_api_proto::google::firestore::v1::Value>,
    {
        self.0.end_at = Some(Cursor {
            values: values.into_iter().collect(),
            before: true,
        });
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.0.offset = offset;
        self
    }

    pub fn order_by<I>(mut self, order_by: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<structured_query::Order>,
    {
        self.0.order_by = order_by.into_iter().map(Into::into).collect();
        self
    }

    pub fn select<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<structured_query::FieldReference>,
    {
        self.0.select = Some(structured_query::Projection {
            fields: fields.into_iter().map(Into::into).collect(),
        });
        self
    }

    pub fn start_after<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = google_api_proto::google::firestore::v1::Value>,
    {
        self.0.start_at = Some(Cursor {
            values: values.into_iter().collect(),
            before: false,
        });
        self
    }

    pub fn start_at<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = google_api_proto::google::firestore::v1::Value>,
    {
        self.0.start_at = Some(Cursor {
            values: values.into_iter().collect(),
            before: true,
        });
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
