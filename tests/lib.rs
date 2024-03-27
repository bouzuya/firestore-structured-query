#[cfg(feature = "serde")]
#[test]
fn test_full_example_with_serde() -> firestore_structured_query::Result<()> {
    use firestore_structured_query::{to_value, FieldPath, Filter, Query};
    use google_api_proto::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Cursor, StructuredQuery, Value,
    };

    assert_eq!(
        StructuredQuery::from(
            // or Query::collection_group(...)
            Query::collection("collection_id1")
                .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
                .r#where(Filter::and([
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
                ]))
                .order_by([
                    FieldPath::raw("field1").ascending(),
                    FieldPath::raw("field2").descending(),
                ])
                // .start_after(...)
                .start_at([
                    to_value(&1)?,
                    to_value(&2)?
                ])
                // .end_before(...)
                .end_at([
                    to_value(&1)?,
                    to_value(&2)?,
                ])
                .offset(1_i32)
                .limit(2_i32),
        ),
        StructuredQuery {
            select: Some(structured_query::Projection {
                fields: vec![
                    structured_query::FieldReference {
                        field_path: "field1".to_string(),
                    },
                    structured_query::FieldReference {
                        field_path: "field2".to_string(),
                    },
                ],
            }),
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: false,
            }],
            r#where: Some(structured_query::Filter {
                filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
                    structured_query::CompositeFilter {
                        op: structured_query::composite_filter::Operator::And as i32,
                        filters: vec![
                            // field filters
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field1".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::LessThan
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(1)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field2".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::LessThanOrEqual
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(2)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field3".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::GreaterThan
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(3)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field4".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::GreaterThanOrEqual
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(4)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field5".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::Equal as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(5)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field6".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::NotEqual
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(6)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field7".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::ArrayContains
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(7)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field8".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::In as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::ArrayValue(
                                                    ArrayValue {
                                                        values: vec![Value {
                                                            value_type: Some(ValueType::IntegerValue(8)),
                                                        }],
                                                    },
                                                )),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field9".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::ArrayContainsAny
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::ArrayValue(
                                                    ArrayValue {
                                                        values: vec![Value {
                                                            value_type: Some(ValueType::IntegerValue(9)),
                                                        }],
                                                    },
                                                )),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field10".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::NotIn as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::ArrayValue(
                                                    ArrayValue {
                                                        values: vec![Value {
                                                            value_type: Some(ValueType::IntegerValue(10)),
                                                        }],
                                                    },
                                                )),
                                            }),
                                        },
                                    )
                                ),
                            },
                            // unary filters
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNan as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field11".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNotNan as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field12".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNotNull as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field13".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNull as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field14".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            // composite filters
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::CompositeFilter(
                                        structured_query::CompositeFilter {
                                            op: structured_query::composite_filter::Operator::And as i32,
                                            filters: vec![
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "a".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "b".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                            ],
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::CompositeFilter(
                                        structured_query::CompositeFilter {
                                            op: structured_query::composite_filter::Operator::Or as i32,
                                            filters: vec![
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "a".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "b".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                            ],
                                        },
                                    )
                                ),
                            },
                        ]
                    }
                ))
            }),
            order_by: vec![
                structured_query::Order {
                    field: Some(structured_query::FieldReference {
                        field_path: "field1".to_string(),
                    }),
                    direction: structured_query::Direction::Ascending as i32,
                },
                structured_query::Order {
                    field: Some(structured_query::FieldReference {
                        field_path: "field2".to_string(),
                    }),
                    direction: structured_query::Direction::Descending as i32,
                },
            ],
            start_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: true,
            }),
            end_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: false,
            }),
            offset: 1_i32,
            limit: Some(2_i32),
        }
    );

    assert_eq!(
        StructuredQuery::from(
            Query::collection_group("collection_id1")
                .start_after([
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ])
                .end_before([
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ]),
        ),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: true,
            }],
            r#where: None,
            order_by: vec![],
            start_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: false,
            }),
            end_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: true,
            }),
            offset: 0_i32,
            limit: None
        }
    );

    Ok(())
}

#[test]
fn test_full_example() -> firestore_structured_query::Result<()> {
    use firestore_structured_query::{FieldPath, Filter, Query};
    use google_api_proto::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Cursor, StructuredQuery, Value,
    };

    assert_eq!(
        StructuredQuery::from(
            // or Query::collection_group(...)
            Query::collection("collection_id1")
                .select([FieldPath::raw("field1"), FieldPath::raw("field2")])
                .r#where(Filter::and([
                    // field filters
                    FieldPath::raw("field1").less_than(Value { value_type: Some(ValueType::IntegerValue(1)) })?,
                    FieldPath::raw("field2").less_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(2)) })?,
                    FieldPath::raw("field3").greater_than(Value { value_type: Some(ValueType::IntegerValue(3)) })?,
                    FieldPath::raw("field4").greater_than_or_equal(Value { value_type: Some(ValueType::IntegerValue(4)) })?,
                    FieldPath::raw("field5").equal(Value { value_type: Some(ValueType::IntegerValue(5)) })?,
                    FieldPath::raw("field6").not_equal(Value { value_type: Some(ValueType::IntegerValue(6)) })?,
                    FieldPath::raw("field7").array_contains(Value { value_type: Some(ValueType::IntegerValue(7)) })?,
                    FieldPath::raw("field8").r#in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(8)) }] })) })?,
                    FieldPath::raw("field9").array_contains_any(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(9)) }] })) })?,
                    FieldPath::raw("field10").not_in(Value { value_type: Some(ValueType::ArrayValue(ArrayValue { values: vec![Value { value_type: Some(ValueType::IntegerValue(10)) }] })) })?,
                    // unary filters
                    FieldPath::raw("field11").is_nan()?,
                    FieldPath::raw("field12").is_not_nan()?,
                    FieldPath::raw("field13").is_not_null()?,
                    FieldPath::raw("field14").is_null()?,
                    // composite filters
                    Filter::and([
                        FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
                        FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
                    ]),
                    Filter::or([
                        FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("a".to_string())) })?,
                        FieldPath::raw("f").equal(Value { value_type: Some(ValueType::StringValue("b".to_string())) })?,
                    ]),
                ]))
                .order_by([
                    FieldPath::raw("field1").ascending(),
                    FieldPath::raw("field2").descending(),
                ])
                // .start_after(...)
                .start_at([
                    Value { value_type: Some(ValueType::IntegerValue(1))},
                    Value { value_type: Some(ValueType::IntegerValue(2))},
                ])
                // .end_before(...)
                .end_at([
                    Value { value_type: Some(ValueType::IntegerValue(1))},
                    Value { value_type: Some(ValueType::IntegerValue(2))},
                ])
                .offset(1_i32)
                .limit(2_i32),
        ),
        StructuredQuery {
            select: Some(structured_query::Projection {
                fields: vec![
                    structured_query::FieldReference {
                        field_path: "field1".to_string(),
                    },
                    structured_query::FieldReference {
                        field_path: "field2".to_string(),
                    },
                ],
            }),
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: false,
            }],
            r#where: Some(structured_query::Filter {
                filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
                    structured_query::CompositeFilter {
                        op: structured_query::composite_filter::Operator::And as i32,
                        filters: vec![
                            // field filters
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field1".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::LessThan
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(1)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field2".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::LessThanOrEqual
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(2)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field3".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::GreaterThan
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(3)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field4".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::GreaterThanOrEqual
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(4)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field5".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::Equal as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(5)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field6".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::NotEqual
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(6)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field7".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::ArrayContains
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::IntegerValue(7)),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field8".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::In as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::ArrayValue(
                                                    ArrayValue {
                                                        values: vec![Value {
                                                            value_type: Some(ValueType::IntegerValue(8)),
                                                        }],
                                                    },
                                                )),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field9".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::ArrayContainsAny
                                                as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::ArrayValue(
                                                    ArrayValue {
                                                        values: vec![Value {
                                                            value_type: Some(ValueType::IntegerValue(9)),
                                                        }],
                                                    },
                                                )),
                                            }),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::FieldFilter(
                                        structured_query::FieldFilter {
                                            field: Some(structured_query::FieldReference {
                                                field_path: "field10".to_string(),
                                            }),
                                            op: structured_query::field_filter::Operator::NotIn as i32,
                                            value: Some(Value {
                                                value_type: Some(ValueType::ArrayValue(
                                                    ArrayValue {
                                                        values: vec![Value {
                                                            value_type: Some(ValueType::IntegerValue(10)),
                                                        }],
                                                    },
                                                )),
                                            }),
                                        },
                                    )
                                ),
                            },
                            // unary filters
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNan as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field11".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNotNan as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field12".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNotNull as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field13".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::UnaryFilter(
                                        structured_query::UnaryFilter {
                                            op: structured_query::unary_filter::Operator::IsNull as i32,
                                            operand_type: Some(structured_query::unary_filter::OperandType::Field(
                                                structured_query::FieldReference {
                                                    field_path: "field14".to_string(),
                                                },
                                            )),
                                        },
                                    )
                                ),
                            },
                            // composite filters
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::CompositeFilter(
                                        structured_query::CompositeFilter {
                                            op: structured_query::composite_filter::Operator::And as i32,
                                            filters: vec![
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "a".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "b".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                            ],
                                        },
                                    )
                                ),
                            },
                            structured_query::Filter {
                                filter_type: Some(
                                    structured_query::filter::FilterType::CompositeFilter(
                                        structured_query::CompositeFilter {
                                            op: structured_query::composite_filter::Operator::Or as i32,
                                            filters: vec![
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "a".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                                structured_query::Filter {
                                                    filter_type: Some(
                                                        structured_query::filter::FilterType::FieldFilter(
                                                            structured_query::FieldFilter {
                                                                field: Some(structured_query::FieldReference {
                                                                    field_path: "f".to_string(),
                                                                }),
                                                                op: structured_query::field_filter::Operator::Equal
                                                                    as i32,
                                                                value: Some(Value {
                                                                    value_type: Some(ValueType::StringValue(
                                                                        "b".to_string(),
                                                                    )),
                                                                }),
                                                            },
                                                        )
                                                    ),
                                                },
                                            ],
                                        },
                                    )
                                ),
                            },
                        ]
                    }
                ))
            }),
            order_by: vec![
                structured_query::Order {
                    field: Some(structured_query::FieldReference {
                        field_path: "field1".to_string(),
                    }),
                    direction: structured_query::Direction::Ascending as i32,
                },
                structured_query::Order {
                    field: Some(structured_query::FieldReference {
                        field_path: "field2".to_string(),
                    }),
                    direction: structured_query::Direction::Descending as i32,
                },
            ],
            start_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: true,
            }),
            end_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: false,
            }),
            offset: 1_i32,
            limit: Some(2_i32),
            find_nearest: None,
        }
    );

    assert_eq!(
        StructuredQuery::from(
            Query::collection_group("collection_id1")
                .start_after([
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ])
                .end_before([
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ]),
        ),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: true,
            }],
            r#where: None,
            order_by: vec![],
            start_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: false,
            }),
            end_at: Some(Cursor {
                values: vec![
                    Value {
                        value_type: Some(ValueType::IntegerValue(1)),
                    },
                    Value {
                        value_type: Some(ValueType::IntegerValue(2)),
                    },
                ],
                before: true,
            }),
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );

    Ok(())
}
