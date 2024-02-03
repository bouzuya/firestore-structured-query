#[test]
fn test_error() -> firestore_structured_query::Result<()> {
    // Added: Error
    use firestore_structured_query::{to_value, Error};
    fn assert_impl<T: std::error::Error + Send + Sync>(_: T) {}
    let e: Error = to_value(&1_u64).unwrap_err();
    assert_impl(e);
    match to_value(&1_u64).unwrap_err() {
        Error::ToValue(e) => {
            assert_eq!(e.to_string(), "u64 is not supported");
        }
    }
    Ok(())
}
