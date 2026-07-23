# whichnet

Detect the current network's identity on Linux and macOS.

`whichnet` reports the Wi-Fi SSID when connected. When SSID is not available (wired ethernet, no wireless interface, or on macOS without Location Services permission), it falls back to the default gateway's MAC address. If neither can be determined, it reports unknown.

## Why

Useful for anything that needs to know "which network am I on right now" without relying on shelling out to `iwgetid`, `nmcli`, or `networksetup`. All lookups go through native platform APIs: `nl80211` netlink on Linux, CoreWLAN on macOS.

## Install

```bash
cargo add whichnet
```

## Usage

```rust
use whichnet::{current_identity, NetworkIdentity};

match current_identity() {
    NetworkIdentity::Ssid(ssid) => println!("On Wi-Fi network: {ssid}"),
    NetworkIdentity::GatewayMac { mac, gateway_ip } => {
        println!("SSID unavailable, gateway MAC: {mac:02x?}, gateway IP: {gateway_ip:?}");
    }
    NetworkIdentity::Unknown => println!("Could not determine network identity"),
}
```

## Platform notes

**Linux**: SSID is read via a netlink socket to `nl80211`. Requires a wireless interface associated to an access point; falls back to gateway MAC otherwise.

**macOS**: SSID is read via CoreWLAN (`CWInterface`). This requires the process to hold Location Services authorization, a macOS restriction on any app reading the SSID. Without it, `whichnet` falls back to gateway MAC. There is no workaround that bypasses this permission.

**Windows**: not supported.

## What "identity" means here

Gateway MAC is a weaker signal than SSID. Two different networks sharing an unconfigured or default router configuration could have the same gateway MAC. It is used only as a fallback when SSID cannot be read, and only when the gateway MAC itself could actually be resolved (an unresolved/zero MAC is treated the same as no signal, and reported as unknown).

## License

MIT
