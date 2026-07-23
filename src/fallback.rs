pub(crate) struct NetdevFallback;

/// `netdev` documents that `mac_addr` may be the all-zero address when the
/// OS could not actually resolve the gateway's hardware address. Treat that
/// as "unresolved" rather than a real MAC.
fn is_unresolved(mac: [u8; 6]) -> bool {
    mac == [0u8; 6]
}

impl NetdevFallback {
    pub(crate) fn gateway_mac(&self) -> Option<([u8; 6], Option<std::net::IpAddr>)> {
        let gateway = netdev::get_default_gateway().ok()?;
        let mac = gateway.mac_addr.octets();
        if is_unresolved(mac) {
            return None;
        }
        let gateway_ip = gateway
            .ipv4
            .first()
            .map(|ip| std::net::IpAddr::V4(*ip))
            .or_else(|| gateway.ipv6.first().map(|ip| std::net::IpAddr::V6(*ip)));
        Some((mac, gateway_ip))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gateway_mac_returns_a_value_or_none_without_panicking() {
        // This is an environment-dependent integration check, not a
        // deterministic assert (CI runners vary in network setup).
        // It only verifies the call doesn't panic and the shape is sane.
        let result = NetdevFallback.gateway_mac();
        if let Some((mac, _gateway_ip)) = result {
            assert_eq!(mac.len(), 6);
        }
    }

    #[test]
    fn all_zero_mac_is_rejected_as_unresolved() {
        assert!(is_unresolved([0, 0, 0, 0, 0, 0]));
        assert!(!is_unresolved([1, 2, 3, 4, 5, 6]));
        assert!(!is_unresolved([0, 0, 0, 0, 0, 1]));
    }
}
