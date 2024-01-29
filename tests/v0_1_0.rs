#[test]
fn test_field_path_filter_ext_array_contains() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::array_contains
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = FieldPath::raw("field7").array_contains(&7)?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field7".to_string()
                    }),
                    op: structured_query::field_filter::Operator::ArrayContains as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(7))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_array_contains_any() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::array_contains_any
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Value,
    };
    let filter1 = FieldPath::raw("field9").array_contains_any(&[9])?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field9".to_string()
                    }),
                    op: structured_query::field_filter::Operator::ArrayContainsAny as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![Value {
                                value_type: Some(ValueType::IntegerValue(9))
                            }]
                        }))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::equal
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = FieldPath::raw("field5").equal(&5)?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field5".to_string()
                    }),
                    op: structured_query::field_filter::Operator::Equal as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(5))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_greater_than() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::greater_than
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = FieldPath::raw("field3").greater_than(&3)?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field3".to_string()
                    }),
                    op: structured_query::field_filter::Operator::GreaterThan as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(3))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_greater_than_or_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::greater_than_or_equal
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = FieldPath::raw("field4").greater_than_or_equal(&4)?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field4".to_string()
                    }),
                    op: structured_query::field_filter::Operator::GreaterThanOrEqual as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(4))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_in() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::r#in
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Value,
    };
    let filter1 = FieldPath::raw("field8").r#in(&[8])?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field8".to_string()
                    }),
                    op: structured_query::field_filter::Operator::In as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![Value {
                                value_type: Some(ValueType::IntegerValue(8))
                            }]
                        }))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_less_than() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::less_than
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = FieldPath::raw("field1").less_than(&1)?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field1".to_string()
                    }),
                    op: structured_query::field_filter::Operator::LessThan as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(1))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_less_than_or_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::less_than_or_equal
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = FieldPath::raw("field2").less_than_or_equal(&2)?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field2".to_string()
                    }),
                    op: structured_query::field_filter::Operator::LessThanOrEqual as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(2))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_not_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::not_equal
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = FieldPath::raw("field6").not_equal(&6)?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field6".to_string()
                    }),
                    op: structured_query::field_filter::Operator::NotEqual as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(6))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_filter_ext_not_in() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::not_in
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Value,
    };
    let filter1 = FieldPath::raw("field10").not_in(&[10])?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field10".to_string()
                    }),
                    op: structured_query::field_filter::Operator::NotIn as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::ArrayValue(ArrayValue {
                            values: vec![Value {
                                value_type: Some(ValueType::IntegerValue(10))
                            }]
                        }))
                    })
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_raw() {
    // Added: FieldPath::raw
    use firestore_structured_query::FieldPath;
    let field_path1 = FieldPath::raw("field1");
    let field_path2 = FieldPath::raw("field1".to_string());
    assert_eq!(field_path1, field_path2);
}

#[test]
fn test_filter_and() -> firestore_structured_query::Result<()> {
    // Added: Filter::and
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _, Filter};
    use google_api_proto::google::firestore::v1::structured_query;
    let filter1 = FieldPath::raw("field1").less_than(&1)?;
    let filter2 = FieldPath::raw("field2").less_than_or_equal(&2)?;
    let filter3 = Filter::and([filter1.clone(), filter2.clone()]);
    assert_eq!(
        structured_query::Filter::from(filter3),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
                structured_query::CompositeFilter {
                    op: structured_query::composite_filter::Operator::And as i32,
                    filters: vec![
                        structured_query::Filter::from(filter1),
                        structured_query::Filter::from(filter2)
                    ],
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_filter_or() -> firestore_structured_query::Result<()> {
    // Added: Filter::or
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _, Filter};
    use google_api_proto::google::firestore::v1::structured_query;
    let filter1 = FieldPath::raw("field1").less_than(&1)?;
    let filter2 = FieldPath::raw("field2").less_than_or_equal(&2)?;
    let filter3 = Filter::or([filter1.clone(), filter2.clone()]);
    assert_eq!(
        structured_query::Filter::from(filter3),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
                structured_query::CompositeFilter {
                    op: structured_query::composite_filter::Operator::Or as i32,
                    filters: vec![
                        structured_query::Filter::from(filter1),
                        structured_query::Filter::from(filter2)
                    ],
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_impl_from_filter_for_structured_query_filter() -> firestore_structured_query::Result<()> {
    // Added: impl From<Filter> for structured_query::Filter
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
    use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    let filter1 = structured_query::Filter::from(FieldPath::raw("field1").less_than(&1)?);
    assert_eq!(
        filter1,
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field1".to_string()
                    }),
                    op: structured_query::field_filter::Operator::LessThan as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(1))
                    })
                },
            )),
        }
    );
    Ok(())
}
