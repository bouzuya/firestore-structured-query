#[test]
fn test_error() -> firestore_structured_query::Result<()> {
    // Added: Error type is now Send + Sync
    use firestore_structured_query::to_value;
    fn assert_impl<T: std::error::Error + Send + Sync>(_: T) {}
    let e = to_value(&1_u64).unwrap_err();
    assert_impl(e);
    Ok(())
}
