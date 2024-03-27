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
    ///         find_nearest: None,
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
            find_nearest: None,
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
    ///         find_nearest: None,
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
            find_nearest: None,
        })
    }

    /// Sets the specified value to end_at and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.google.firestore.v1.Cursor.google.firestore.v1.StructuredQuery.end_at>
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.Cursor>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_end_at() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{
    ///     structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    /// };
    /// let query1 = Query::collection_group("collection_id1").end_at([
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(1)),
    ///     },
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(2)),
    ///     },
    /// ]);
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
    ///         end_at: Some(Cursor {
    ///             values: vec![
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(1)),
    ///                 },
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(2)),
    ///                 },
    ///             ],
    ///             before: false,
    ///         }),
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Sets the specified value to end_at and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.google.firestore.v1.Cursor.google.firestore.v1.StructuredQuery.end_at>
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.Cursor>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_end_before() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{
    ///     structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    /// };
    /// let query1 = Query::collection_group("collection_id1").end_before([
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(1)),
    ///     },
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(2)),
    ///     },
    /// ]);
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
    ///         end_at: Some(Cursor {
    ///             values: vec![
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(1)),
    ///                 },
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(2)),
    ///                 },
    ///             ],
    ///             before: true,
    ///         }),
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Sets the specified value to limit and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.google.protobuf.Int32Value.google.firestore.v1.StructuredQuery.limit>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_limit() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    /// let query1 = Query::collection_group("collection_id1").limit(1_i32);
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
    ///         limit: Some(1_i32),
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn limit(mut self, limit: i32) -> Self {
        self.0.limit = Some(limit);
        self
    }

    /// Sets the specified value to offset and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.int32.google.firestore.v1.StructuredQuery.offset>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_offset() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    /// let query1 = Query::collection_group("collection_id1").offset(1_i32);
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
    ///         offset: 1_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn offset(mut self, offset: i32) -> Self {
        self.0.offset = offset;
        self
    }

    /// Sets the specified value to order_by and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.repeated.google.firestore.v1.StructuredQuery.Order.google.firestore.v1.StructuredQuery.order_by>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_order_by() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::{FieldPath, Query};
    /// use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    /// let query1 = Query::collection_group("collection_id1").order_by([
    ///     FieldPath::raw("field1").ascending(),
    ///     FieldPath::raw("field2").descending(),
    /// ]);
    /// assert_eq!(
    ///     StructuredQuery::from(query1),
    ///     StructuredQuery {
    ///         select: None,
    ///         from: vec![structured_query::CollectionSelector {
    ///             collection_id: "collection_id1".to_string(),
    ///             all_descendants: true,
    ///         }],
    ///         r#where: None,
    ///         order_by: vec![
    ///             structured_query::Order::from(FieldPath::raw("field1").ascending()),
    ///             structured_query::Order::from(FieldPath::raw("field2").descending()),
    ///         ],
    ///         start_at: None,
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// let order_by1 = vec![
    ///     structured_query::Order {
    ///         field: Some(structured_query::FieldReference {
    ///             field_path: "field1".to_string(),
    ///         }),
    ///         direction: structured_query::Direction::Ascending as i32,
    ///     },
    ///     structured_query::Order {
    ///         field: Some(structured_query::FieldReference {
    ///             field_path: "field2".to_string(),
    ///         }),
    ///         direction: structured_query::Direction::Descending as i32,
    ///     },
    /// ];
    /// let query2 = Query::collection_group("collection_id1").order_by(order_by1.clone());
    /// assert_eq!(
    ///     StructuredQuery::from(query2),
    ///     StructuredQuery {
    ///         select: None,
    ///         from: vec![structured_query::CollectionSelector {
    ///             collection_id: "collection_id1".to_string(),
    ///             all_descendants: true,
    ///         }],
    ///         r#where: None,
    ///         order_by: order_by1,
    ///         start_at: None,
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
    pub fn order_by<I>(mut self, order_by: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<structured_query::Order>,
    {
        self.0.order_by = order_by.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the specified value to select and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.google.firestore.v1.StructuredQuery.Projection.google.firestore.v1.StructuredQuery.select>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_select() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::{FieldPath, Query};
    /// use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    /// let query1 = Query::collection_group("collection_id1")
    ///     .select([FieldPath::raw("field1"), FieldPath::raw("field2")]);
    /// assert_eq!(
    ///     StructuredQuery::from(query1),
    ///     StructuredQuery {
    ///         select: Some(structured_query::Projection {
    ///             fields: vec![
    ///                 structured_query::FieldReference {
    ///                     field_path: "field1".to_string(),
    ///                 },
    ///                 structured_query::FieldReference {
    ///                     field_path: "field2".to_string(),
    ///                 },
    ///             ],
    ///         }),
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
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Sets the specified value to start_at and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.google.firestore.v1.Cursor.google.firestore.v1.StructuredQuery.start_at>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_start_after() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{
    ///     structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    /// };
    /// let query1 = Query::collection_group("collection_id1").start_after([
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(1)),
    ///     },
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(2)),
    ///     },
    /// ]);
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
    ///         start_at: Some(Cursor {
    ///             values: vec![
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(1)),
    ///                 },
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(2)),
    ///                 },
    ///             ],
    ///             before: false,
    ///         }),
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Sets the specified value to start_at and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.google.firestore.v1.Cursor.google.firestore.v1.StructuredQuery.start_at>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_start_at() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::Query;
    /// use google_api_proto::google::firestore::v1::{
    ///     structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    /// };
    /// let query1 = Query::collection_group("collection_id1").start_at([
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(1)),
    ///     },
    ///     Value {
    ///         value_type: Some(ValueType::IntegerValue(2)),
    ///     },
    /// ]);
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
    ///         start_at: Some(Cursor {
    ///             values: vec![
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(1)),
    ///                 },
    ///                 Value {
    ///                     value_type: Some(ValueType::IntegerValue(2)),
    ///                 },
    ///             ],
    ///             before: true,
    ///         }),
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Sets the specified value to where and returns the Query.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.FIELDS.google.firestore.v1.StructuredQuery.Filter.google.firestore.v1.StructuredQuery.where>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_query_where() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::{FieldPath, Query};
    /// use google_api_proto::google::firestore::v1::{structured_query, StructuredQuery};
    /// let query1 =
    ///     Query::collection_group("collection_id1").r#where(FieldPath::raw("field1").is_nan()?);
    /// assert_eq!(
    ///     StructuredQuery::from(query1),
    ///     StructuredQuery {
    ///         select: None,
    ///         from: vec![structured_query::CollectionSelector {
    ///             collection_id: "collection_id1".to_string(),
    ///             all_descendants: true,
    ///         }],
    ///         r#where: Some(structured_query::Filter {
    ///             filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
    ///                 structured_query::UnaryFilter {
    ///                     op: structured_query::unary_filter::Operator::IsNan as i32,
    ///                     operand_type: Some(structured_query::unary_filter::OperandType::Field(
    ///                         structured_query::FieldReference {
    ///                             field_path: "field1".to_string(),
    ///                         }
    ///                     ))
    ///                 }
    ///             ))
    ///         }),
    ///         order_by: vec![],
    ///         start_at: None,
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// let filter1 = structured_query::Filter {
    ///     filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
    ///         structured_query::UnaryFilter {
    ///             op: structured_query::unary_filter::Operator::IsNan as i32,
    ///             operand_type: Some(structured_query::unary_filter::OperandType::Field(
    ///                 structured_query::FieldReference {
    ///                     field_path: "field1".to_string(),
    ///                 },
    ///             )),
    ///         },
    ///     )),
    /// };
    /// let query2 = Query::collection_group("collection_id1").r#where(filter1.clone());
    /// assert_eq!(
    ///     StructuredQuery::from(query2),
    ///     StructuredQuery {
    ///         select: None,
    ///         from: vec![structured_query::CollectionSelector {
    ///             collection_id: "collection_id1".to_string(),
    ///             all_descendants: true,
    ///         }],
    ///         r#where: Some(filter1),
    ///         order_by: vec![],
    ///         start_at: None,
    ///         end_at: None,
    ///         offset: 0_i32,
    ///         limit: None,
    ///         find_nearest: None,
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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
