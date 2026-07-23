pub(crate) struct NetdevFallback;

impl NetdevFallback {
    pub(crate) fn gateway_mac(&self) -> Option<([u8; 6], Option<std::net::IpAddr>)> {
        let gateway = netdev::get_default_gateway().ok()?;
        let mac = gateway.mac_addr.octets();
        let subnet = gateway
            .ipv4
            .first()
            .map(|ip| std::net::IpAddr::V4(*ip))
            .or_else(|| gateway.ipv6.first().map(|ip| std::net::IpAddr::V6(*ip)));
        Some((mac, subnet))
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
        if let Some((mac, _subnet)) = result {
            assert_eq!(mac.len(), 6);
        }
    }
}
