use tcx_solana::address::SolanaAddress;

#[test]
fn test_solana_address_creation() {
    let addr_str = "11111111111111111111111111111111";
    let addr = SolanaAddress::from_string(addr_str).unwrap();
    assert_eq!(addr.to_string(), addr_str);
}

#[test]
fn test_solana_address_bytes() {
    let addr_str = "11111111111111111111111111111111";
    let addr = SolanaAddress::from_string(addr_str).unwrap();
    let bytes = addr.to_bytes();
    assert_eq!(bytes.len(), 32);
}

#[test]
fn test_solana_address_invalid() {
    let invalid_addr = "invalid_solana_address";
    assert!(SolanaAddress::from_string(invalid_addr).is_err());
}

#[test]
fn test_solana_address_display() {
    let addr_str = "11111111111111111111111111111111";
    let addr = SolanaAddress::from_string(addr_str).unwrap();
    let displayed = format!("{}", addr);
    assert_eq!(displayed, addr_str);
}
