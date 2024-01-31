#[test]
fn test_field_path_filter_ext_is_nan() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::is_nan
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
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
fn test_field_path_filter_ext_is_not_nan() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::is_not_nan
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
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
fn test_field_path_filter_ext_is_not_null() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::is_not_null
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
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
fn test_field_path_filter_ext_is_null() -> firestore_structured_query::Result<()> {
    // Added: FieldPathFilterExt::is_null
    use firestore_structured_query::{FieldPath, FieldPathFilterExt as _};
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
