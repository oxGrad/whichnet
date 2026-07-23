pub(crate) struct Nl80211Source;

impl Nl80211Source {
    pub(crate) fn ssid(&self) -> Option<String> {
        let mut socket = nl80211::Socket::connect().ok()?;
        let interfaces = socket.get_interfaces_info().ok()?;
        interfaces
            .into_iter()
            .find_map(|iface| iface.ssid)
            .and_then(|bytes| String::from_utf8(bytes).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssid_lookup_does_not_panic() {
        // Environment-dependent: whether this returns Some or None depends
        // on whether the CI runner has a wireless interface associated to
        // an AP. This only verifies the call path doesn't panic.
        let _ = Nl80211Source.ssid();
    }
}
