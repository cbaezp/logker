use logker::response;

#[test]
fn test_send_custom_response() {
    let result = response::send_custom_response("192.168.1.1".to_string(), "Blocked!");
    assert!(result.is_ok());
}
