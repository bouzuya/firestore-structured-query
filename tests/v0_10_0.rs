#![allow(missing_docs)]

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
    use firestore_structured_query::{FieldPath, Filter};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    let filter1 = FieldPath::raw("field1").less_than(Value {
        value_type: Some(ValueType::IntegerValue(1)),
    })?;
    let filter2 = FieldPath::raw("field2").less_than_or_equal(Value {
        value_type: Some(ValueType::IntegerValue(2)),
    })?;
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
    use firestore_structured_query::{FieldPath, Filter};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    let filter1 = FieldPath::raw("field1").less_than(Value {
        value_type: Some(ValueType::IntegerValue(1)),
    })?;
    let filter2 = FieldPath::raw("field2").less_than_or_equal(Value {
        value_type: Some(ValueType::IntegerValue(2)),
    })?;
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
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    let filter1 = structured_query::Filter::from(FieldPath::raw("field1").less_than(Value {
        value_type: Some(ValueType::IntegerValue(1)),
    })?);
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

#[test]
fn test_impl_from_field_path_for_field_reference() -> firestore_structured_query::Result<()> {
    // Added: impl From<FieldPath> for FieldReference
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
    let field_path1 = FieldPath::raw("field1");
    assert_eq!(
        structured_query::FieldReference::from(field_path1),
        structured_query::FieldReference {
            field_path: "field1".to_string(),
        }
    );
    Ok(())
}

#[test]
fn test_error() -> firestore_structured_query::Result<()> {
    // Added: Error
    use firestore_structured_query::{Error, FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::Value;
    fn assert_impl<T: std::error::Error + Send + Sync>(_: T) {}
    struct S;
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Err(Error::new("S is not supported"))
        }
    }
    let e: Error = FieldPath::raw("field1").equal(S).unwrap_err();
    assert_impl(e);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_error_to_value_variant() -> firestore_structured_query::Result<()> {
    // Added: Error::ToValue when the `serde` feature is enabled.
    use firestore_structured_query::{to_value, Error};
    let e: Error = to_value(&1_u64).unwrap_err();
    assert_eq!(e.to_string(), "u64 is not supported");
    Ok(())
}

#[test]
fn test_field_path_array_contains() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::array_contains
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field7").array_contains(Value {
        value_type: Some(ValueType::IntegerValue(7)),
    })?;
    let filter2 = FieldPath::raw("field7").array_contains(S(7))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field7".to_string(),
                }),
                op: structured_query::field_filter::Operator::ArrayContains as i32,
                value: Some(Value {
                    value_type: Some(ValueType::IntegerValue(7)),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_array_contains_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::array_contains (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Value,
    };
    struct S(Vec<i64>);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: self
                        .0
                        .into_iter()
                        .map(|i| Value {
                            value_type: Some(ValueType::IntegerValue(i)),
                        })
                        .collect(),
                })),
            })
        }
    }
    let filter1 = FieldPath::raw("field9").array_contains_any(Value {
        value_type: Some(ValueType::ArrayValue(ArrayValue {
            values: vec![Value {
                value_type: Some(ValueType::IntegerValue(9)),
            }],
        })),
    })?;
    let filter2 = FieldPath::raw("field9").array_contains_any(S(vec![9]))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field9".to_string(),
                }),
                op: structured_query::field_filter::Operator::ArrayContainsAny as i32,
                value: Some(Value {
                    value_type: Some(ValueType::ArrayValue(ArrayValue {
                        values: vec![Value {
                            value_type: Some(ValueType::IntegerValue(9)),
                        }],
                    })),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_array_contains_any_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::array_contains_any (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
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
    use firestore_structured_query::{FieldPath, Order};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
    let order: Order = FieldPath::raw("field1").ascending();
    assert_eq!(
        structured_query::Order::from(order),
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
    use firestore_structured_query::{FieldPath, Order};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
    let order: Order = FieldPath::raw("field1").descending();
    assert_eq!(
        structured_query::Order::from(order),
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field5").equal(Value {
        value_type: Some(ValueType::IntegerValue(5)),
    })?;
    let filter2 = FieldPath::raw("field5").equal(S(5))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field5".to_string(),
                }),
                op: structured_query::field_filter::Operator::Equal as i32,
                value: Some(Value {
                    value_type: Some(ValueType::IntegerValue(5)),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_equal_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::equal (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field3").greater_than(Value {
        value_type: Some(ValueType::IntegerValue(3)),
    })?;
    let filter2 = FieldPath::raw("field3").greater_than(S(3))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field3".to_string(),
                }),
                op: structured_query::field_filter::Operator::GreaterThan as i32,
                value: Some(Value {
                    value_type: Some(ValueType::IntegerValue(3)),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_greater_than_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::greater_than (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field4").greater_than_or_equal(Value {
        value_type: Some(ValueType::IntegerValue(4)),
    })?;
    let filter2 = FieldPath::raw("field4").greater_than_or_equal(S(4))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field4".to_string(),
                }),
                op: structured_query::field_filter::Operator::GreaterThanOrEqual as i32,
                value: Some(Value {
                    value_type: Some(ValueType::IntegerValue(4)),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_greater_than_or_equal_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::greater_than_or_equal (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Value,
    };
    struct S(Vec<i64>);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: self
                        .0
                        .into_iter()
                        .map(|i| Value {
                            value_type: Some(ValueType::IntegerValue(i)),
                        })
                        .collect(),
                })),
            })
        }
    }
    let filter1 = FieldPath::raw("field8").r#in(Value {
        value_type: Some(ValueType::ArrayValue(ArrayValue {
            values: vec![Value {
                value_type: Some(ValueType::IntegerValue(8)),
            }],
        })),
    })?;
    let filter2 = FieldPath::raw("field8").r#in(S(vec![8]))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field8".to_string(),
                }),
                op: structured_query::field_filter::Operator::In as i32,
                value: Some(Value {
                    value_type: Some(ValueType::ArrayValue(ArrayValue {
                        values: vec![Value {
                            value_type: Some(ValueType::IntegerValue(8)),
                        }],
                    })),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_in_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::r#in (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
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
fn test_field_path_is_nan() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::is_nan
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
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
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
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
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
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
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
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

#[test]
fn test_field_path_less_than() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::less_than
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field1").less_than(Value {
        value_type: Some(ValueType::IntegerValue(1)),
    })?;
    let filter2 = FieldPath::raw("field1").less_than(S(1))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field1".to_string(),
                }),
                op: structured_query::field_filter::Operator::LessThan as i32,
                value: Some(Value {
                    value_type: Some(ValueType::IntegerValue(1)),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_less_than_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::less_than (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field2").less_than_or_equal(Value {
        value_type: Some(ValueType::IntegerValue(2)),
    })?;
    let filter2 = FieldPath::raw("field2").less_than_or_equal(S(2))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field2".to_string(),
                }),
                op: structured_query::field_filter::Operator::LessThanOrEqual as i32,
                value: Some(Value {
                    value_type: Some(ValueType::IntegerValue(2)),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_less_than_or_equal_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::less_than_or_equal (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field6").not_equal(Value {
        value_type: Some(ValueType::IntegerValue(6)),
    })?;
    let filter2 = FieldPath::raw("field6").not_equal(S(6))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field6".to_string(),
                }),
                op: structured_query::field_filter::Operator::NotEqual as i32,
                value: Some(Value {
                    value_type: Some(ValueType::IntegerValue(6)),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_not_equal_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::not_equal (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Value,
    };
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
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, ArrayValue, Value,
    };
    struct S(Vec<i64>);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::ArrayValue(ArrayValue {
                    values: self
                        .0
                        .into_iter()
                        .map(|i| Value {
                            value_type: Some(ValueType::IntegerValue(i)),
                        })
                        .collect(),
                })),
            })
        }
    }
    let filter1 = FieldPath::raw("field10").not_in(Value {
        value_type: Some(ValueType::ArrayValue(ArrayValue {
            values: vec![Value {
                value_type: Some(ValueType::IntegerValue(10)),
            }],
        })),
    })?;
    let filter2 = FieldPath::raw("field10").not_in(S(vec![10]))?;
    let expected = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference {
                    field_path: "field10".to_string(),
                }),
                op: structured_query::field_filter::Operator::NotIn as i32,
                value: Some(Value {
                    value_type: Some(ValueType::ArrayValue(ArrayValue {
                        values: vec![Value {
                            value_type: Some(ValueType::IntegerValue(10)),
                        }],
                    })),
                }),
            },
        )),
    };
    assert_eq!(structured_query::Filter::from(filter1), expected);
    assert_eq!(structured_query::Filter::from(filter2), expected);
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_field_path_not_in_serialize() -> firestore_structured_query::Result<()> {
    // Added: FieldPath::not_in (for T: Serialize)
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
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
fn test_into_value() -> firestore_structured_query::Result<()> {
    // Added: IntoValue
    use firestore_structured_query::{FieldPath, IntoValue, Result};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    struct S(i64);
    impl IntoValue for S {
        fn into_value(self) -> Result<Value> {
            Ok(Value {
                value_type: Some(ValueType::IntegerValue(self.0)),
            })
        }
    }
    let filter1 = FieldPath::raw("field1").equal(S(1))?;
    assert_eq!(
        structured_query::Filter::from(filter1),
        structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference {
                        field_path: "field1".to_string()
                    }),
                    op: structured_query::field_filter::Operator::Equal as i32,
                    value: Some(Value {
                        value_type: Some(ValueType::IntegerValue(1))
                    })
                },
            ))
        }
    );
    Ok(())
}

#[test]
fn test_order() -> firestore_structured_query::Result<()> {
    // Added: Order
    use firestore_structured_query::{FieldPath, Order};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
    let order: Order = FieldPath::raw("field1").ascending();
    assert_eq!(
        structured_query::Order::from(order),
        structured_query::Order {
            field: Some(structured_query::FieldReference {
                field_path: "field1".to_string()
            }),
            direction: structured_query::Direction::Ascending as i32
        }
    );
    let order: Order = FieldPath::raw("field1").descending();
    assert_eq!(
        structured_query::Order::from(order),
        structured_query::Order {
            field: Some(structured_query::FieldReference {
                field_path: "field1".to_string()
            }),
            direction: structured_query::Direction::Descending as i32
        }
    );
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_to_value() -> firestore_structured_query::Result<()> {
    // Added: ::to_value
    use firestore_structured_query::to_value;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{value::ValueType, Value};
    assert_eq!(
        to_value(&1_i64)?,
        Value {
            value_type: Some(ValueType::IntegerValue(1_i64)),
        }
    );
    Ok(())
}
#[test]
fn test_field_path_new() {
    // Fixed: FieldPath::new
    use firestore_structured_query::FieldPath;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::structured_query;
    let field_path1 = FieldPath::new(["field1"]);
    assert_eq!(
        structured_query::FieldReference::from(field_path1),
        structured_query::FieldReference {
            field_path: "field1".to_string(),
        }
    );
    let field_path2 = FieldPath::new(["field1", "field2"]);
    assert_eq!(
        structured_query::FieldReference::from(field_path2),
        structured_query::FieldReference {
            field_path: "field1.field2".to_string(),
        }
    );
    let field_path3 = FieldPath::new(["foo", "x&y"]);
    assert_eq!(
        structured_query::FieldReference::from(field_path3),
        structured_query::FieldReference {
            field_path: "foo.`x&y`".to_string(),
        }
    );
    let field_path4 = FieldPath::new(["a`b", r#"a\b"#]);
    assert_eq!(
        structured_query::FieldReference::from(field_path4),
        structured_query::FieldReference {
            field_path: r#"`a\`b`.`a\\b`"#.to_string(),
        }
    );
}

#[test]
fn test_query_collection() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::collection includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, StructuredQuery,
    };
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
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_collection_group() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::collection_group includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, StructuredQuery,
    };
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
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_end_at() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::end_at includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    };
    let query1 = Query::collection_group("collection_id1").end_at([
        Value {
            value_type: Some(ValueType::IntegerValue(1)),
        },
        Value {
            value_type: Some(ValueType::IntegerValue(2)),
        },
    ]);
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
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_end_before() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::end_before includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    };
    let query1 = Query::collection_group("collection_id1").end_before([
        Value {
            value_type: Some(ValueType::IntegerValue(1)),
        },
        Value {
            value_type: Some(ValueType::IntegerValue(2)),
        },
    ]);
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

#[test]
fn test_query_limit() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::limit includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, StructuredQuery,
    };
    let query1 = Query::collection_group("collection_id1").limit(1_i32);
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
            limit: Some(1_i32),
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_offset() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::offset includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, StructuredQuery,
    };
    let query1 = Query::collection_group("collection_id1").offset(1_i32);
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
            offset: 1_i32,
            limit: None,
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_order_by() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::order_by includes StructuredQuery::find_nearest.
    use firestore_structured_query::{FieldPath, Query};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, StructuredQuery,
    };
    let query1 = Query::collection_group("collection_id1").order_by([
        FieldPath::raw("field1").ascending(),
        FieldPath::raw("field2").descending(),
    ]);
    assert_eq!(
        StructuredQuery::from(query1),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: true,
            }],
            r#where: None,
            order_by: vec![
                structured_query::Order::from(FieldPath::raw("field1").ascending()),
                structured_query::Order::from(FieldPath::raw("field2").descending()),
            ],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    let order_by1 = vec![
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
    ];
    let query2 = Query::collection_group("collection_id1").order_by(order_by1.clone());
    assert_eq!(
        StructuredQuery::from(query2),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: true,
            }],
            r#where: None,
            order_by: order_by1,
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_select() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::select includes StructuredQuery::find_nearest.
    use firestore_structured_query::{FieldPath, Query};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, StructuredQuery,
    };
    let query1 = Query::collection_group("collection_id1")
        .select([FieldPath::raw("field1"), FieldPath::raw("field2")]);
    assert_eq!(
        StructuredQuery::from(query1),
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
                all_descendants: true,
            }],
            r#where: None,
            order_by: vec![],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_start_after() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::start_after includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    };
    let query1 = Query::collection_group("collection_id1").start_after([
        Value {
            value_type: Some(ValueType::IntegerValue(1)),
        },
        Value {
            value_type: Some(ValueType::IntegerValue(2)),
        },
    ]);
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
            end_at: None,
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_start_at() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::start_at includes StructuredQuery::find_nearest.
    use firestore_structured_query::Query;
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, value::ValueType, Cursor, StructuredQuery, Value,
    };
    let query1 = Query::collection_group("collection_id1").start_at([
        Value {
            value_type: Some(ValueType::IntegerValue(1)),
        },
        Value {
            value_type: Some(ValueType::IntegerValue(2)),
        },
    ]);
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
            end_at: None,
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    Ok(())
}

#[test]
fn test_query_where() -> firestore_structured_query::Result<()> {
    // Changed: The output of Query::r#where includes StructuredQuery::find_nearest.
    use firestore_structured_query::{FieldPath, Query};
    use googleapis_tonic_google_firestore_v1::google::firestore::v1::{
        structured_query, StructuredQuery,
    };
    let query1 =
        Query::collection_group("collection_id1").r#where(FieldPath::raw("field1").is_nan()?);
    assert_eq!(
        StructuredQuery::from(query1),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: true,
            }],
            r#where: Some(structured_query::Filter {
                filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
                    structured_query::UnaryFilter {
                        op: structured_query::unary_filter::Operator::IsNan as i32,
                        operand_type: Some(structured_query::unary_filter::OperandType::Field(
                            structured_query::FieldReference {
                                field_path: "field1".to_string(),
                            }
                        ))
                    }
                ))
            }),
            order_by: vec![],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    let filter1 = structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
            structured_query::UnaryFilter {
                op: structured_query::unary_filter::Operator::IsNan as i32,
                operand_type: Some(structured_query::unary_filter::OperandType::Field(
                    structured_query::FieldReference {
                        field_path: "field1".to_string(),
                    },
                )),
            },
        )),
    };
    let query2 = Query::collection_group("collection_id1").r#where(filter1.clone());
    assert_eq!(
        StructuredQuery::from(query2),
        StructuredQuery {
            select: None,
            from: vec![structured_query::CollectionSelector {
                collection_id: "collection_id1".to_string(),
                all_descendants: true,
            }],
            r#where: Some(filter1),
            order_by: vec![],
            start_at: None,
            end_at: None,
            offset: 0_i32,
            limit: None,
            find_nearest: None,
        }
    );
    Ok(())
}
