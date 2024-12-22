
use std::error::Error;

pub fn block_ip(ip: &str, duration: u64) -> Result<(), Box<dyn Error>> {
    
    let ipt = iptables::new(false)?; // `false` specifies IPv4 (use `true` for IPv6)
    
    ipt.append("filter", "INPUT", &format!("-s {} -j DROP", ip))?;
    
    println!("Blocked IP: {} for {} seconds", ip, duration);
    Ok(())
}
