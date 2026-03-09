use tcx_bitcoin_bdk::address::BitcoinAddress;

#[test]
fn test_bitcoin_address_creation() {
    let addr_str = "1A1z7agoat2LWQLZLV37ZLX4My6ps6nFX";
    let addr = BitcoinAddress::from_string(addr_str, bitcoin::Network::Bitcoin).unwrap();
    assert_eq!(addr.to_string(), addr_str);
}

#[test]
fn test_bitcoin_address_type() {
    let addr_str = "1A1z7agoat2LWQLZLV37ZLX4My6ps6nFX";
    let addr = BitcoinAddress::from_string(addr_str, bitcoin::Network::Bitcoin).unwrap();
    assert_eq!(
        addr.address_type(),
        tcx_bitcoin_bdk::address::AddressType::Legacy
    );
}

#[test]
fn test_bitcoin_address_invalid() {
    let invalid_addr = "invalid_bitcoin_address";
    assert!(BitcoinAddress::from_string(invalid_addr, bitcoin::Network::Bitcoin).is_err());
}

#[test]
fn test_bitcoin_address_display() {
    let addr_str = "1A1z7agoat2LWQLZLV37ZLX4My6ps6nFX";
    let addr = BitcoinAddress::from_string(addr_str, bitcoin::Network::Bitcoin).unwrap();
    let displayed = format!("{}", addr);
    assert_eq!(displayed, addr_str);
}
