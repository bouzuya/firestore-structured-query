use crate::error::Result;
use crate::field_path::FieldPath;
use crate::to_value;

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

pub trait FieldPathFilterExt {
    fn array_contains<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn array_contains_any<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn greater_than<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn greater_than_or_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn r#in<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn is_nan(&self) -> Result<Filter>;

    fn is_not_nan(&self) -> Result<Filter>;

    fn is_null(&self) -> Result<Filter>;

    fn is_not_null(&self) -> Result<Filter>;

    fn less_than<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn less_than_or_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn not_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;

    fn not_in<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize;
}

impl FieldPathFilterExt for FieldPath {
    fn array_contains<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::ArrayContains, value)
    }

    fn array_contains_any<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(
            self.clone(),
            field_filter::Operator::ArrayContainsAny,
            value,
        )
    }

    fn equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::Equal, value)
    }

    fn greater_than<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::GreaterThan, value)
    }

    fn greater_than_or_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(
            self.clone(),
            field_filter::Operator::GreaterThanOrEqual,
            value,
        )
    }

    fn r#in<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::In, value)
    }

    fn is_nan(&self) -> Result<Filter> {
        unary_op(self.clone(), unary_filter::Operator::IsNan)
    }

    fn is_not_nan(&self) -> Result<Filter> {
        unary_op(self.clone(), unary_filter::Operator::IsNotNan)
    }

    fn is_not_null(&self) -> Result<Filter> {
        unary_op(self.clone(), unary_filter::Operator::IsNotNull)
    }

    fn is_null(&self) -> Result<Filter> {
        unary_op(self.clone(), unary_filter::Operator::IsNull)
    }

    fn less_than<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::LessThan, value)
    }

    fn less_than_or_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::LessThanOrEqual, value)
    }

    fn not_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::NotEqual, value)
    }

    fn not_in<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        op(self.clone(), field_filter::Operator::NotIn, value)
    }
}

fn op<T>(field_path: FieldPath, op: field_filter::Operator, value: &T) -> Result<Filter>
where
    T: serde::Serialize,
{
    Ok(Filter(structured_query::Filter {
        filter_type: Some(structured_query::filter::FilterType::FieldFilter(
            structured_query::FieldFilter {
                field: Some(structured_query::FieldReference::from(field_path)),
                op: op as i32,
                value: Some(to_value(value)?),
            },
        )),
    }))
}

fn unary_op(field_path: FieldPath, op: unary_filter::Operator) -> Result<Filter> {
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
