use crate::field_path::FieldPath;
use crate::{error::Result, IntoValue};

use google_api_proto::google::firestore::v1::structured_query::{self, field_filter, unary_filter};

/// A Firestore query filter.
///
/// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#filter>
///
/// # Examples
///
/// ```rust
/// # fn example_filter() -> firestore_structured_query::Result<()> {
/// use firestore_structured_query::{FieldPath, Filter};
/// use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
/// let filter1: Filter = FieldPath::raw("field1").less_than(Value {
///     value_type: Some(ValueType::IntegerValue(1)),
/// })?;
/// let filter2: Filter = FieldPath::raw("field2").less_than_or_equal(Value {
///     value_type: Some(ValueType::IntegerValue(2)),
/// })?;
/// let filter3 = Filter::and([filter1.clone(), filter2.clone()]);
/// let filter4 = Filter::or([filter1.clone(), filter2.clone()]);
/// assert_eq!(
///     structured_query::Filter::from(filter3),
///     structured_query::Filter {
///         filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
///             structured_query::CompositeFilter {
///                 op: structured_query::composite_filter::Operator::And as i32,
///                 filters: vec![
///                     structured_query::Filter::from(filter1),
///                     structured_query::Filter::from(filter2)
///                 ],
///             },
///         )),
///     }
/// );
/// assert_eq!(
///     structured_query::Filter::from(filter4),
///     structured_query::Filter {
///         filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
///             structured_query::CompositeFilter {
///                 op: structured_query::composite_filter::Operator::Or as i32,
///                 filters: vec![
///                     structured_query::Filter::from(filter1),
///                     structured_query::Filter::from(filter2)
///                 ],
///             },
///         )),
///     }
/// );
/// #     Ok(())
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Filter(structured_query::Filter);

impl Filter {
    /// Creates a new `CompositeFilter` with the `And` operator.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#compositefilter>
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.CompositeFilter.Operator.ENUM_VALUES.google.firestore.v1.StructuredQuery.CompositeFilter.Operator.AND>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_filter_and() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::{FieldPath, Filter};
    /// use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    /// let filter1 = FieldPath::raw("field1").less_than(Value {
    ///     value_type: Some(ValueType::IntegerValue(1)),
    /// })?;
    /// let filter2 = FieldPath::raw("field2").less_than_or_equal(Value {
    ///     value_type: Some(ValueType::IntegerValue(2)),
    /// })?;
    /// let filter3 = Filter::and([filter1.clone(), filter2.clone()]);
    /// assert_eq!(
    ///     structured_query::Filter::from(filter3),
    ///     structured_query::Filter {
    ///         filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
    ///             structured_query::CompositeFilter {
    ///                 op: structured_query::composite_filter::Operator::And as i32,
    ///                 filters: vec![
    ///                     structured_query::Filter::from(filter1),
    ///                     structured_query::Filter::from(filter2)
    ///                 ],
    ///             },
    ///         )),
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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

    /// Creates a new `CompositeFilter` with the `Or` operator.
    ///
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#compositefilter>
    /// <https://firebase.google.com/docs/firestore/reference/rpc/google.firestore.v1#google.firestore.v1.StructuredQuery.CompositeFilter.Operator.ENUM_VALUES.google.firestore.v1.StructuredQuery.CompositeFilter.Operator.OR>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn test_filter_or() -> firestore_structured_query::Result<()> {
    /// use firestore_structured_query::{FieldPath, Filter};
    /// use google_api_proto::google::firestore::v1::{structured_query, value::ValueType, Value};
    /// let filter1 = FieldPath::raw("field1").less_than(Value {
    ///     value_type: Some(ValueType::IntegerValue(1)),
    /// })?;
    /// let filter2 = FieldPath::raw("field2").less_than_or_equal(Value {
    ///     value_type: Some(ValueType::IntegerValue(2)),
    /// })?;
    /// let filter3 = Filter::or([filter1.clone(), filter2.clone()]);
    /// assert_eq!(
    ///     structured_query::Filter::from(filter3),
    ///     structured_query::Filter {
    ///         filter_type: Some(structured_query::filter::FilterType::CompositeFilter(
    ///             structured_query::CompositeFilter {
    ///                 op: structured_query::composite_filter::Operator::Or as i32,
    ///                 filters: vec![
    ///                     structured_query::Filter::from(filter1),
    ///                     structured_query::Filter::from(filter2)
    ///                 ],
    ///             },
    ///         )),
    ///     }
    /// );
    /// #     Ok(())
    /// # }
    /// ```
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
