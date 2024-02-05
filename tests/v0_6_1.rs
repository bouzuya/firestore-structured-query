#[test]
fn test_impl_x_for_order() -> firestore_structured_query::Result<()> {
    // Added: impl Clone for Order
    // Added: impl Debug for Order
    // Added: impl PartialEq for Order
    use firestore_structured_query::Order;
    fn assert_impl<T: Clone + std::fmt::Debug + PartialEq>() {}
    assert_impl::<Order>();
    Ok(())
}
