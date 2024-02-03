use google_api_proto::google::firestore::v1::structured_query::{self, field_filter, unary_filter};

use crate::error::Result;
use crate::{Filter, Order};

/// A Firestore Field Path.
///
/// <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
///
/// > - Must separate field names with a single period (`.`)
/// > - May be passed as a dot-delimited (`.`) string of segments where each segment is either a simple field name or a quoted field name (defined below).
/// >
/// > A simple field name is one where all of the following are true:
/// >
/// > - Contains only the characters `a-z`, `A-Z`, `0-9`, and underscore (`_`)
/// > - Does not start with `0-9`
/// >
/// > A quoted field name starts and ends with the backtick character (`` ` ``). For example, `` foo.`x&y` `` refers to the `x&y` field nested under the `foo` field. To construct a field name with the backtick character, escape the backtick character with the backslash character (`\`). For convenience, you can avoid quoted field names by passing the field path as a FieldPath object (for example, see JavaScript FieldPath).
///
/// # Examples
///
/// ```rust
/// use firestore_structured_query::FieldPath;
/// use google_api_proto::google::firestore::v1::structured_query;
///
/// let field_path1 = FieldPath::raw("field1");
/// assert_eq!(
///     structured_query::FieldReference::from(field_path1),
///     structured_query::FieldReference {
///         field_path: "field1".to_string(),
///     }
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FieldPath {
    field_names: Vec<String>,
}

impl FieldPath {
    /// Creates a new field path without escaping.
    pub fn raw<S>(field_path: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            field_names: vec![field_path.into()],
        }
    }
}

// for Filter
impl FieldPath {
    pub fn array_contains<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::ArrayContains, value)
    }

    pub fn array_contains_any<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(
            self.clone(),
            field_filter::Operator::ArrayContainsAny,
            value,
        )
    }

    pub fn equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::Equal, value)
    }

    pub fn greater_than<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::GreaterThan, value)
    }

    pub fn greater_than_or_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(
            self.clone(),
            field_filter::Operator::GreaterThanOrEqual,
            value,
        )
    }

    pub fn r#in<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::In, value)
    }

    pub fn is_nan(&self) -> Result<Filter> {
        Filter::unary(self.clone(), unary_filter::Operator::IsNan)
    }

    pub fn is_not_nan(&self) -> Result<Filter> {
        Filter::unary(self.clone(), unary_filter::Operator::IsNotNan)
    }

    pub fn is_not_null(&self) -> Result<Filter> {
        Filter::unary(self.clone(), unary_filter::Operator::IsNotNull)
    }

    pub fn is_null(&self) -> Result<Filter> {
        Filter::unary(self.clone(), unary_filter::Operator::IsNull)
    }

    pub fn less_than<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::LessThan, value)
    }

    pub fn less_than_or_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::LessThanOrEqual, value)
    }

    pub fn not_equal<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::NotEqual, value)
    }

    pub fn not_in<T>(&self, value: &T) -> Result<Filter>
    where
        T: serde::Serialize,
    {
        Filter::field(self.clone(), field_filter::Operator::NotIn, value)
    }
}

// for Order
impl FieldPath {
    pub fn ascending(&self) -> Order {
        Order::new(self.clone(), structured_query::Direction::Ascending)
    }

    pub fn descending(&self) -> Order {
        Order::new(self.clone(), structured_query::Direction::Descending)
    }
}

impl std::convert::From<FieldPath> for structured_query::FieldReference {
    fn from(field_path: FieldPath) -> Self {
        structured_query::FieldReference {
            field_path: field_path.field_names.join("."),
        }
    }
}
