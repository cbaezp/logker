use logker::logging;

#[test]
fn test_init_logging() {
    let result = logging::init_logging();
    assert!(result.is_ok());
}
