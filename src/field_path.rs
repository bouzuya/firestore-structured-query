use google_api_proto::google::firestore::v1::structured_query;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FieldPath {
    field_names: Vec<String>,
}

impl FieldPath {
    pub fn raw<S>(field_path: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            field_names: vec![field_path.into()],
        }
    }

    pub(crate) fn to_field_reference(&self) -> structured_query::FieldReference {
        structured_query::FieldReference {
            field_path: self.field_names.join("."),
        }
    }
}

impl std::convert::From<FieldPath> for structured_query::FieldReference {
    fn from(field_path: FieldPath) -> Self {
        field_path.to_field_reference()
    }
}
