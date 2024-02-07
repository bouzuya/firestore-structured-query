use google_api_proto::google::firestore::v1::{structured_query, Cursor, StructuredQuery};

/// A Firestore query.
///
/// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#structuredquery>
///
/// # Examples
///
/// ```rust
/// # fn example_mod_doc() -> firestore_structured_query::Result<()> {
/// use firestore_structured_query::{FieldPath, Filter, Query};
/// use google_api_proto::google::firestore::v1::{
///     structured_query, value::ValueType, ArrayValue, Cursor, StructuredQuery, Value,
/// };
///
/// let _ = StructuredQuery::from(
///     // or Query::collection_group(...)
///     Query::collection("collection_id1")
///         .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
///         .r#where(Filter::and([
///             // field filters
///             FieldPath::raw("field1").less_than(Value { value_type: Some(ValueType::IntegerValue(1)) })?,
///             FieldPath::raw("field2").less_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(2)) })?,
///             FieldPath::raw("field3").greater_than(Value { value_type: Some(ValueType::IntegerValue(3)) })?,
///             FieldPath::raw("field4").greater_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(4)) })?,
///             FieldPath::raw("field5").equal(Value { value_type: Some(ValueType::IntegerValue(5)) })?,
///             FieldPath::raw("field6").not_equal(Value { value_type: Some(ValueType::IntegerValue(6)) })?,
///             FieldPath::raw("field7").array_contains(Value { value_type: Some(ValueType::IntegerValue(7)) })?,
///             FieldPath::raw("field8").r#in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(8)) }] })) })?,
///             FieldPath::raw("field9").array_contains_any(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(9)) }] })) })?,
///             FieldPath::raw("field10").not_in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(10)) }] })) })?,
///             // unary filters
///             FieldPath::raw("field11").is_nan()?,
///             FieldPath::raw("field12").is_not_nan()?,
///             FieldPath::raw("field13").is_not_null()?,
///             FieldPath::raw("field14").is_null()?,
///             // composite filters
///             Filter::and([
///                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
///                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
///             ]),
///             Filter::or([
///                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
///                 FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
///             ]),
///         ]))
///         .order_by([
///             FieldPath::raw("field1").ascending(),
///             FieldPath::raw("field2").descending(),
///         ])
///         // .start_after(...)
///         .start_at([
///             Value { value_type: Some(ValueType::IntegerValue(1))},
///             Value { value_type: Some(ValueType::IntegerValue(2))},
///         ])
///         // .end_before(...)
///         .end_at([
///             Value { value_type: Some(ValueType::IntegerValue(1))},
///             Value { value_type: Some(ValueType::IntegerValue(2))},
///         ])
///         .offset(1_i32)
///         .limit(2_i32),
/// );
///
/// // If "serde" feature is enabled.
/// use firestore_structured_query::to_value;
/// let _ = StructuredQuery::from(
///     // or Query::collection_group(...)
///     Query::collection("collection_id1")
///         .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
///         .r#where(Filter::and([
///             // field filters
///             FieldPath::raw("field1").less_than(&1)?,
///             FieldPath::raw("field2").less_than_or_equal(&2)?,
///             FieldPath::raw("field3").greater_than(&3)?,
///             FieldPath::raw("field4").greater_than_or_equal(&4)?,
///             FieldPath::raw("field5").equal(&5)?,
///             FieldPath::raw("field6").not_equal(&6)?,
///             FieldPath::raw("field7").array_contains(&7)?,
///             FieldPath::raw("field8").r#in(&[8])?,
///             FieldPath::raw("field9").array_contains_any(&[9])?,
///             FieldPath::raw("field10").not_in(&[10])?,
///             // unary filters
///             FieldPath::raw("field11").is_nan()?,
///             FieldPath::raw("field12").is_not_nan()?,
///             FieldPath::raw("field13").is_not_null()?,
///             FieldPath::raw("field14").is_null()?,
///             // composite filters
///             Filter::and([
///                 FieldPath::raw("f").equal(&"a")?,
///                 FieldPath::raw("f").equal(&"b")?,
///             ]),
///             Filter::or([
///                 FieldPath::raw("f").equal(&"a")?,
///                 FieldPath::raw("f").equal(&"b")?,
///             ]),
///         ]))
///         .order_by([
///             FieldPath::raw("field1").ascending(),
///             FieldPath::raw("field2").descending(),
///         ])
///         // .start_after(...)
///         .start_at([
///             to_value(&1)?,
///             to_value(&2)?
///         ])
///         // .end_before(...)
///         .end_at([
///             to_value(&1)?,
///             to_value(&2)?,
///         ])
///         .offset(1_i32)
///         .limit(2_i32),
/// );
/// #     Ok(())
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Query(StructuredQuery);

impl Query {
    /// Creates a new `Query` for a collection.
    ///
    /// The query that internally holds a `CollectionSelector` with `all_descendants` set to `false`.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.CollectionSelector>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_collection() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    /// let query1 = Query::collection("collection_id1");
    /// assert_eq!(
    ///     StructuredQuery::from(query1),
    ///     StructuredQuery {
    ///         select: None,
    ///         from: vec![structured_query::CollectionSelector {
    ///             collection_id: "collection_id1".to_string(),
    ///             all_descendants: false,
    ///         }],
    ///         r#where: None,
    ///         order_by: vec![],
    ///         start_at: None,
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Creates a new `Query` for a collection group.
    ///
    /// The query that internally holds a `CollectionSelector` with `all_descendants` set to `true`.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.CollectionSelector>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_collection_group() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    /// let query1 = Query::collection_group("collection_id1");
    /// assert_eq!(
    ///     StructuredQuery::from(query1),
    ///     StructuredQuery {
    ///         select: None,
    ///         from: vec![structured_query::CollectionSelector {
    ///             collection_id: "collection_id1".to_string(),
    ///             all_descendants: true,
    ///         }],
    ///         r#where: None,
    ///         order_by: vec![],
    ///         start_at: None,
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    pub fn limit(mut self, limit: i32) -> Self {
        self.0.limit = Some(limit);
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
