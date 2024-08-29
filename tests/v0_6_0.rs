// Deprecated: Replace google-api-proto with googleapis-tonic-google-firestore-v1.
// use google_api_proto::google::firestore::v1::structured_query;

// #[test]
// fn test_error() -> firestore_structured_query::Result<()> {
//     // Added: Error
//     use firestore_structured_query::{Error, FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::Value;
//     fn assert_impl<T: std::error::Error + Send + Sync>(_: T) {}
//     struct S;
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Err(Error::new("S is not supported"))
//         }
//     }
//     let e: Error = FieldPath::raw("field1").equal(S).unwrap_err();
//     assert_impl(e);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_error_to_value_variant() -> firestore_structured_query::Result<()> {
//     // Added: Error::ToValue when the `serde` feature is enabled.
//     use firestore_structured_query::{to_value, Error};
//     let e: Error = to_value(&1_u64).unwrap_err();
//     assert_eq!(e.to_string(), "u64 is not supported");
//     Ok(())
// }

// #[test]
// fn test_field_path_array_contains() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::array_contains
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field7").array_contains(Value {
//         value_type: Some(ValueType::IntegerValue(7)),
//     })?;
//     let filter2 = FieldPath::raw("field7").array_contains(S(7))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field7".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::ArrayContains as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::IntegerValue(7)),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_array_contains_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::array_contains (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     let filter1 = FieldPath::raw("field7").array_contains(&7)?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field7".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::ArrayContains as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(7))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_array_contains_any() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::array_contains_any
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, ArrayValue, Value,
//     };
//     struct S(Vec<i64>);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::ArrayValue(ArrayValue {
//                     values: self
//                         .0
//                         .into_iter()
//                         .map(|i| Value {
//                             value_type: Some(ValueType::IntegerValue(i)),
//                         })
//                         .collect(),
//                 })),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field9").array_contains_any(Value {
//         value_type: Some(ValueType::ArrayValue(ArrayValue {
//             values: vec![Value {
//                 value_type: Some(ValueType::IntegerValue(9)),
//             }],
//         })),
//     })?;
//     let filter2 = FieldPath::raw("field9").array_contains_any(S(vec![9]))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field9".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::ArrayContainsAny as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::ArrayValue(ArrayValue {
//                         values: vec![Value {
//                             value_type: Some(ValueType::IntegerValue(9)),
//                         }],
//                     })),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_array_contains_any_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::array_contains_any (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, ArrayValue, Value,
//     };
//     let filter1 = FieldPath::raw("field9").array_contains_any(&[9])?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field9".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::ArrayContainsAny as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::IntegerValue(9))
//                             }]
//                         }))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_ascending() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::ascending
//     use firestore_structured_query::{FieldPath, Order};
//     use google_api_proto::google::firestore::v1::structured_query;
//     let order: Order = FieldPath::raw("field1").ascending();
//     assert_eq!(
//         structured_query::Order::from(order),
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field1".to_string()
//             }),
//             direction: structured_query::Direction::Ascending as i32
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_descending() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::descending
//     use firestore_structured_query::{FieldPath, Order};
//     use google_api_proto::google::firestore::v1::structured_query;
//     let order: Order = FieldPath::raw("field1").descending();
//     assert_eq!(
//         structured_query::Order::from(order),
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field1".to_string()
//             }),
//             direction: structured_query::Direction::Descending as i32
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_equal() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::equal
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field5").equal(Value {
//         value_type: Some(ValueType::IntegerValue(5)),
//     })?;
//     let filter2 = FieldPath::raw("field5").equal(S(5))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field5".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::Equal as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::IntegerValue(5)),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_equal_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::equal (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     let filter1 = FieldPath::raw("field5").equal(&5)?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field5".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::Equal as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(5))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_greater_than() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::greater_than
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field3").greater_than(Value {
//         value_type: Some(ValueType::IntegerValue(3)),
//     })?;
//     let filter2 = FieldPath::raw("field3").greater_than(S(3))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field3".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::GreaterThan as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::IntegerValue(3)),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_greater_than_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::greater_than (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     let filter1 = FieldPath::raw("field3").greater_than(&3)?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field3".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::GreaterThan as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(3))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_greater_than_or_equal() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::greater_than_or_equal
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field4").greater_than_or_equal(Value {
//         value_type: Some(ValueType::IntegerValue(4)),
//     })?;
//     let filter2 = FieldPath::raw("field4").greater_than_or_equal(S(4))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field4".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::GreaterThanOrEqual as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::IntegerValue(4)),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_greater_than_or_equal_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::greater_than_or_equal (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     let filter1 = FieldPath::raw("field4").greater_than_or_equal(&4)?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field4".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::GreaterThanOrEqual as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(4))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_in() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::r#in
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, ArrayValue, Value,
//     };
//     struct S(Vec<i64>);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::ArrayValue(ArrayValue {
//                     values: self
//                         .0
//                         .into_iter()
//                         .map(|i| Value {
//                             value_type: Some(ValueType::IntegerValue(i)),
//                         })
//                         .collect(),
//                 })),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field8").r#in(Value {
//         value_type: Some(ValueType::ArrayValue(ArrayValue {
//             values: vec![Value {
//                 value_type: Some(ValueType::IntegerValue(8)),
//             }],
//         })),
//     })?;
//     let filter2 = FieldPath::raw("field8").r#in(S(vec![8]))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field8".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::In as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::ArrayValue(ArrayValue {
//                         values: vec![Value {
//                             value_type: Some(ValueType::IntegerValue(8)),
//                         }],
//                     })),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_in_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::r#in (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, ArrayValue, Value,
//     };
//     let filter1 = FieldPath::raw("field8").r#in(&[8])?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field8".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::In as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::IntegerValue(8))
//                             }]
//                         }))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_is_nan() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::is_nan
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::structured_query;
//     let filter1 = FieldPath::raw("field11").is_nan()?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
//                 structured_query::UnaryFilter {
//                     op: structured_query::unary_filter::Operator::IsNan as i32,
//                     operand_type: Some(structured_query::unary_filter::OperandType::Field(
//                         structured_query::FieldReference {
//                             field_path: "field11".to_string()
//                         }
//                     )),
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_is_not_nan() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::is_not_nan
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::structured_query;
//     let filter1 = FieldPath::raw("field12").is_not_nan()?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
//                 structured_query::UnaryFilter {
//                     op: structured_query::unary_filter::Operator::IsNotNan as i32,
//                     operand_type: Some(structured_query::unary_filter::OperandType::Field(
//                         structured_query::FieldReference {
//                             field_path: "field12".to_string()
//                         }
//                     )),
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_is_not_null() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::is_not_null
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::structured_query;
//     let filter1 = FieldPath::raw("field13").is_not_null()?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
//                 structured_query::UnaryFilter {
//                     op: structured_query::unary_filter::Operator::IsNotNull as i32,
//                     operand_type: Some(structured_query::unary_filter::OperandType::Field(
//                         structured_query::FieldReference {
//                             field_path: "field13".to_string()
//                         }
//                     )),
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_is_null() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::is_null
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::structured_query;
//     let filter1 = FieldPath::raw("field14").is_null()?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
//                 structured_query::UnaryFilter {
//                     op: structured_query::unary_filter::Operator::IsNull as i32,
//                     operand_type: Some(structured_query::unary_filter::OperandType::Field(
//                         structured_query::FieldReference {
//                             field_path: "field14".to_string()
//                         }
//                     )),
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_less_than() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::less_than
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field1").less_than(Value {
//         value_type: Some(ValueType::IntegerValue(1)),
//     })?;
//     let filter2 = FieldPath::raw("field1").less_than(S(1))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field1".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::LessThan as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::IntegerValue(1)),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_less_than_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::less_than (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     let filter1 = FieldPath::raw("field1").less_than(&1)?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field1".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::LessThan as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(1))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_less_than_or_equal() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::less_than_or_equal
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field2").less_than_or_equal(Value {
//         value_type: Some(ValueType::IntegerValue(2)),
//     })?;
//     let filter2 = FieldPath::raw("field2").less_than_or_equal(S(2))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field2".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::LessThanOrEqual as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::IntegerValue(2)),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_less_than_or_equal_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::less_than_or_equal (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     let filter1 = FieldPath::raw("field2").less_than_or_equal(&2)?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field2".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::LessThanOrEqual as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(2))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_not_equal() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::not_equal
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field6").not_equal(Value {
//         value_type: Some(ValueType::IntegerValue(6)),
//     })?;
//     let filter2 = FieldPath::raw("field6").not_equal(S(6))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field6".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::NotEqual as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::IntegerValue(6)),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_not_equal_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::not_equal (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
//     let filter1 = FieldPath::raw("field6").not_equal(&6)?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field6".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::NotEqual as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(6))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_field_path_not_in() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::not_in
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, ArrayValue, Value,
//     };
//     struct S(Vec<i64>);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::ArrayValue(ArrayValue {
//                     values: self
//                         .0
//                         .into_iter()
//                         .map(|i| Value {
//                             value_type: Some(ValueType::IntegerValue(i)),
//                         })
//                         .collect(),
//                 })),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field10").not_in(Value {
//         value_type: Some(ValueType::ArrayValue(ArrayValue {
//             values: vec![Value {
//                 value_type: Some(ValueType::IntegerValue(10)),
//             }],
//         })),
//     })?;
//     let filter2 = FieldPath::raw("field10").not_in(S(vec![10]))?;
//     let expected = structured_query::Filter {
//         filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//             structured_query::FieldFilter {
//                 field: Some(structured_query::FieldReference {
//                     field_path: "field10".to_string(),
//                 }),
//                 op: structured_query::field_filter::Operator::NotIn as i32,
//                 value: Some(Value {
//                     value_type: Some(ValueType::ArrayValue(ArrayValue {
//                         values: vec![Value {
//                             value_type: Some(ValueType::IntegerValue(10)),
//                         }],
//                     })),
//                 }),
//             },
//         )),
//     };
//     assert_eq!(structured_query::Filter::from(filter1), expected);
//     assert_eq!(structured_query::Filter::from(filter2), expected);
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_field_path_not_in_serialize() -> firestore_structured_query::Result<()> {
//     // Added: FieldPath::not_in (for T: Serialize)
//     use firestore_structured_query::FieldPath;
//     use google_api_proto::google::firestore::v1::{
//         structured_query, value::ValueType, ArrayValue, Value,
//     };
//     let filter1 = FieldPath::raw("field10").not_in(&[10])?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field10".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::NotIn as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::ArrayValue(ArrayValue {
//                             values: vec![Value {
//                                 value_type: Some(ValueType::IntegerValue(10))
//                             }]
//                         }))
//                     })
//                 },
//             )),
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_into_value() -> firestore_structured_query::Result<()> {
//     // Added: IntoValue
//     use firestore_structured_query::{FieldPath, IntoValue, Result};
//     use google_api_proto::google::firestore::v1::{value::ValueType, Value};
//     struct S(i64);
//     impl IntoValue for S {
//         fn into_value(self) -> Result<Value> {
//             Ok(Value {
//                 value_type: Some(ValueType::IntegerValue(self.0)),
//             })
//         }
//     }
//     let filter1 = FieldPath::raw("field1").equal(S(1))?;
//     assert_eq!(
//         structured_query::Filter::from(filter1),
//         structured_query::Filter {
//             filter_type: Some(structured_query::filter::FilterType::FieldFilter(
//                 structured_query::FieldFilter {
//                     field: Some(structured_query::FieldReference {
//                         field_path: "field1".to_string()
//                     }),
//                     op: structured_query::field_filter::Operator::Equal as i32,
//                     value: Some(Value {
//                         value_type: Some(ValueType::IntegerValue(1))
//                     })
//                 },
//             ))
//         }
//     );
//     Ok(())
// }

// #[test]
// fn test_order() -> firestore_structured_query::Result<()> {
//     // Added: Order
//     use firestore_structured_query::{FieldPath, Order};
//     use google_api_proto::google::firestore::v1::structured_query;
//     let order: Order = FieldPath::raw("field1").ascending();
//     assert_eq!(
//         structured_query::Order::from(order),
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field1".to_string()
//             }),
//             direction: structured_query::Direction::Ascending as i32
//         }
//     );
//     let order: Order = FieldPath::raw("field1").descending();
//     assert_eq!(
//         structured_query::Order::from(order),
//         structured_query::Order {
//             field: Some(structured_query::FieldReference {
//                 field_path: "field1".to_string()
//             }),
//             direction: structured_query::Direction::Descending as i32
//         }
//     );
//     Ok(())
// }

// #[cfg(feature = "serde")]
// #[test]
// fn test_to_value() -> firestore_structured_query::Result<()> {
//     // Added: ::to_value
//     use firestore_structured_query::to_value;
//     use google_api_proto::google::firestore::v1::{value::ValueType, Value};
//     assert_eq!(
//         to_value(&1_i64)?,
//         Value {
//             value_type: Some(ValueType::IntegerValue(1_i64)),
//         }
//     );
//     Ok(())
// }
