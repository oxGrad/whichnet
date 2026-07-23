use std::net::IpAddr;

mod fallback;
#[cfg(target_os = "linux")]
mod linux;

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkIdentity {
    Ssid(String),
    GatewayMac {
        mac: [u8; 6],
        subnet: Option<IpAddr>,
    },
    Unknown,
}

pub(crate) trait IdentitySource {
    fn ssid(&self) -> Option<String>;
    fn gateway_mac(&self) -> Option<([u8; 6], Option<IpAddr>)>;
}

pub(crate) fn resolve_identity(source: &dyn IdentitySource) -> NetworkIdentity {
    if let Some(ssid) = source.ssid() {
        return NetworkIdentity::Ssid(ssid);
    }
    if let Some((mac, subnet)) = source.gateway_mac() {
        return NetworkIdentity::GatewayMac { mac, subnet };
    }
    NetworkIdentity::Unknown
}

pub fn current_identity() -> NetworkIdentity {
    NetworkIdentity::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSource {
        ssid: Option<String>,
        gateway_mac: Option<([u8; 6], Option<IpAddr>)>,
    }

    impl IdentitySource for MockSource {
        fn ssid(&self) -> Option<String> {
            self.ssid.clone()
        }
        fn gateway_mac(&self) -> Option<([u8; 6], Option<IpAddr>)> {
            self.gateway_mac
        }
    }

    #[test]
    fn prefers_ssid_when_available() {
        let source = MockSource {
            ssid: Some("home-wifi".to_string()),
            gateway_mac: Some(([1, 2, 3, 4, 5, 6], None)),
        };
        assert_eq!(
            resolve_identity(&source),
            NetworkIdentity::Ssid("home-wifi".to_string())
        );
    }

    #[test]
    fn falls_back_to_gateway_mac_when_no_ssid() {
        let source = MockSource {
            ssid: None,
            gateway_mac: Some(([1, 2, 3, 4, 5, 6], Some("192.168.1.1".parse().unwrap()))),
        };
        assert_eq!(
            resolve_identity(&source),
            NetworkIdentity::GatewayMac {
                mac: [1, 2, 3, 4, 5, 6],
                subnet: Some("192.168.1.1".parse().unwrap()),
            }
        );
    }

    #[test]
    fn unknown_when_neither_available() {
        let source = MockSource {
            ssid: None,
            gateway_mac: None,
        };
        assert_eq!(resolve_identity(&source), NetworkIdentity::Unknown);
    }
}
