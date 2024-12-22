#[allow(dead_code)]
pub fn send_custom_response(ip: String, payload: &str) -> Result<(), String> {
    println!("Sending custom response to {} with payload: {}", ip, payload);
    Ok(())
}
