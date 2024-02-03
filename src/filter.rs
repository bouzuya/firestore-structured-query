use crate::field_path::FieldPath;
use crate::{error::Result, IntoValue};

use google_api_proto::google::firestore::v1::structured_query::{self, field_filter, unary_filter};

#[derive(Clone, Debug, PartialEq)]
pub struct Filter(structured_query::Filter);

impl Filter {
    pub fn and<I>(filters: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self(structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
                structured_query::CompositeFilter {
                    op: structured_query::composite_filter::Operator::And as i32,
                    filters: filters.into_iter().map(|f| f.0).collect(),
                },
            )),
        })
    }

    pub fn or<I>(filters: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self(structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
                structured_query::CompositeFilter {
                    op: structured_query::composite_filter::Operator::Or as i32,
                    filters: filters.into_iter().map(|f| f.0).collect(),
                },
            )),
        })
    }
}

impl From<Filter> for structured_query::Filter {
    fn from(filter: Filter) -> Self {
        filter.0
    }
}

impl Filter {
    pub(crate) fn field<T>(
        field_path: FieldPath,
        op: field_filter::Operator,
        value: T,
    ) -> Result<Filter>
    where
        T: IntoValue,
    {
        Ok(Filter(structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::FieldFilter(
                structured_query::FieldFilter {
                    field: Some(structured_query::FieldReference::from(field_path)),
                    op: op as i32,
                    value: Some(value.into_value()?),
                },
            )),
        }))
    }

    pub(crate) fn unary(field_path: FieldPath, op: unary_filter::Operator) -> Result<Filter> {
        Ok(Filter(structured_query::Filter {
            filter_type: Some(structured_query::filter::FilterType::UnaryFilter(
                structured_query::UnaryFilter {
                    op: op as i32,
                    operand_type: Some(unary_filter::OperandType::Field(
                        structured_query::FieldReference::from(field_path),
                    )),
                },
            )),
        }))
    }
}
