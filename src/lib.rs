use std::net::IpAddr;

mod fallback;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkIdentity {
    Ssid(String),
    GatewayMac {
        mac: [u8; 6],
        gateway_ip: Option<IpAddr>,
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
    if let Some((mac, gateway_ip)) = source.gateway_mac() {
        return NetworkIdentity::GatewayMac { mac, gateway_ip };
    }
    NetworkIdentity::Unknown
}

#[cfg(target_os = "linux")]
struct PlatformSource {
    ssid_source: linux::Nl80211Source,
    fallback: fallback::NetdevFallback,
}

#[cfg(target_os = "macos")]
struct PlatformSource {
    ssid_source: macos::CoreWlanSource,
    fallback: fallback::NetdevFallback,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl IdentitySource for PlatformSource {
    fn ssid(&self) -> Option<String> {
        self.ssid_source.ssid()
    }
    fn gateway_mac(&self) -> Option<([u8; 6], Option<IpAddr>)> {
        self.fallback.gateway_mac()
    }
}

#[cfg(target_os = "linux")]
fn platform_source() -> PlatformSource {
    PlatformSource {
        ssid_source: linux::Nl80211Source,
        fallback: fallback::NetdevFallback,
    }
}

#[cfg(target_os = "macos")]
fn platform_source() -> PlatformSource {
    PlatformSource {
        ssid_source: macos::CoreWlanSource,
        fallback: fallback::NetdevFallback,
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn current_identity() -> NetworkIdentity {
    resolve_identity(&platform_source())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
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
                gateway_ip: Some("192.168.1.1".parse().unwrap()),
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

    #[test]
    fn current_identity_does_not_panic() {
        // Environment-dependent end-to-end check across whichever real
        // platform source is compiled in for this OS.
        let _ = current_identity();
    }
}
