use objc2_core_wlan::CWInterface;

pub(crate) struct CoreWlanSource;

impl CoreWlanSource {
    pub(crate) fn ssid(&self) -> Option<String> {
        unsafe {
            let interface = CWInterface::interface();
            interface.ssid().map(|ns_string| ns_string.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssid_lookup_does_not_panic() {
        // Environment-dependent: requires Location Services (TCC)
        // authorization to return Some even when connected to Wi-Fi.
        // This only verifies the call path doesn't panic.
        let _ = CoreWlanSource.ssid();
    }
}
