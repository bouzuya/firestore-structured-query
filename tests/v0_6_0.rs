#[test]
fn test_field_path_array_contains() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::array_contains
    use firestore_structured_query::FieldPath;
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
fn test_field_path_array_contains_any() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::array_contains_any
    use firestore_structured_query::FieldPath;
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
fn test_field_path_ascending() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::ascending
    use firestore_structured_query::FieldPath;
    use google_api_proto::google::firestore::v1::structured_query;
    assert_eq!(
        FieldPath::raw("field1").ascending(),
        structured_query::Order {
            field: Some(structured_query::FieldReference {
                field_path: "field1".to_string()
            }),
            direction: structured_query::Direction::Ascending as i32
        }
    );
    Ok(())
}

#[test]
fn test_field_path_descending() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::descending
    use firestore_structured_query::FieldPath;
    use google_api_proto::google::firestore::v1::structured_query;
    assert_eq!(
        FieldPath::raw("field1").descending(),
        structured_query::Order {
            field: Some(structured_query::FieldReference {
                field_path: "field1".to_string()
            }),
            direction: structured_query::Direction::Descending as i32
        }
    );
    Ok(())
}

#[test]
fn test_field_path_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::equal
    use firestore_structured_query::FieldPath;
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
fn test_field_path_greater_than() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::greater_than
    use firestore_structured_query::FieldPath;
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
fn test_field_path_greater_than_or_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::greater_than_or_equal
    use firestore_structured_query::FieldPath;
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
fn test_field_path_in() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::r#in
    use firestore_structured_query::FieldPath;
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
fn test_field_path_less_than() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::less_than
    use firestore_structured_query::FieldPath;
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
fn test_field_path_less_than_or_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::less_than_or_equal
    use firestore_structured_query::FieldPath;
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
fn test_field_path_not_equal() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::not_equal
    use firestore_structured_query::FieldPath;
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
fn test_field_path_not_in() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::not_in
    use firestore_structured_query::FieldPath;
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

// unary filter

#[test]
fn test_field_path_is_nan() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::is_nan
    use firestore_structured_query::FieldPath;
    use google_api_proto::google::firestore::v1::structured_query;
    let filter1 = FieldPath::raw("field11").is_nan()?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
                structured_query::UnaryFilter {
                    op: structured_query::unary_filter::Operator::IsNan as i32,
                    operand_type: Some(structured_query::unary_filter::OperandType::Field(
                        structured_query::FieldReference {
                            field_path: "field11".to_string()
                        }
                    )),
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_is_not_nan() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::is_not_nan
    use firestore_structured_query::FieldPath;
    use google_api_proto::google::firestore::v1::structured_query;
    let filter1 = FieldPath::raw("field12").is_not_nan()?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
                structured_query::UnaryFilter {
                    op: structured_query::unary_filter::Operator::IsNotNan as i32,
                    operand_type: Some(structured_query::unary_filter::OperandType::Field(
                        structured_query::FieldReference {
                            field_path: "field12".to_string()
                        }
                    )),
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_is_not_null() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::is_not_null
    use firestore_structured_query::FieldPath;
    use google_api_proto::google::firestore::v1::structured_query;
    let filter1 = FieldPath::raw("field13").is_not_null()?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
                structured_query::UnaryFilter {
                    op: structured_query::unary_filter::Operator::IsNotNull as i32,
                    operand_type: Some(structured_query::unary_filter::OperandType::Field(
                        structured_query::FieldReference {
                            field_path: "field13".to_string()
                        }
                    )),
                },
            )),
        }
    );
    Ok(())
}

#[test]
fn test_field_path_is_null() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::is_null
    use firestore_structured_query::FieldPath;
    use google_api_proto::google::firestore::v1::structured_query;
    let filter1 = FieldPath::raw("field14").is_null()?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
                structured_query::UnaryFilter {
                    op: structured_query::unary_filter::Operator::IsNull as i32,
                    operand_type: Some(structured_query::unary_filter::OperandType::Field(
                        structured_query::FieldReference {
                            field_path: "field14".to_string()
                        }
                    )),
                },
            )),
        }
    );
    Ok(())
}
