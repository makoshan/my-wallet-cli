use tcx_evm::address::EvmAddress;

#[test]
fn test_evm_address_creation() {
    let addr_str = "0x5aAeb6053ba3EEdb6A475A1d3E4e77F9E6d3c467";
    let addr = EvmAddress::from_hex(addr_str).unwrap();
    assert!(!addr.to_string().is_empty());
}

#[test]
fn test_evm_address_checksum() {
    let addr_str = "0x5aAeb6053ba3EEdb6A475A1d3E4e77F9E6d3c467";
    let addr = EvmAddress::from_hex(addr_str).unwrap();
    let checksum = addr.to_checksum();
    assert!(checksum.starts_with("0x"));
    assert_eq!(checksum.len(), 42); // 0x + 40 hex chars
}

#[test]
fn test_evm_address_invalid() {
    let invalid_addr = "invalid_address";
    assert!(EvmAddress::from_hex(invalid_addr).is_err());
}

#[test]
fn test_evm_address_short() {
    let short_addr = "0x1234";
    assert!(EvmAddress::from_hex(short_addr).is_err());
}
