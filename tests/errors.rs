use logker::errors::Result;

#[test]
fn test_result_type() {
    let result: Result<()> = Ok(());
    assert!(result.is_ok());

    let result: Result<()> = Err("Error occurred".to_string());
    assert!(result.is_err());
}
