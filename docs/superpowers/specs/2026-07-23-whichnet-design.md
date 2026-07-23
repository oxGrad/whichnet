# whichnet — design spec

## Purpose

Standalone, reusable Rust crate that detects the network identity of the current
device. Originally scoped under the name `hivenet` to support HiveMind's planned
Hive Mode "auto-pause on untrusted network" feature (trusted-networks list in the
dashboard; hive sync/mDNS auto-pauses when the device isn't on a trusted network).
Not tied to HiveMind internals — independently useful/publishable.

## Public API

```rust
pub enum NetworkIdentity {
    Ssid(String),
    GatewayMac { mac: [u8; 6], subnet: Option<IpAddr> },
    Unknown,
}

pub fn current_identity() -> NetworkIdentity;
```

Callers get the strongest identity signal available on the current platform and
network state, without needing to know why a fallback occurred.

## Architecture

- `src/lib.rs` — public API, re-exports `NetworkIdentity`, dispatches to
  platform module via `#[cfg]`.
- `src/linux.rs` (cfg(target_os = "linux")) — SSID via `nl80211` crate.
- `src/macos.rs` (cfg(target_os = "macos")) — SSID via `objc2-core-wlan`.
- `src/fallback.rs` — gateway MAC via `default-net`, shared by both platforms.
- Internal `trait IdentitySource` boundary separating identity-lookup logic from
  the platform FFI/netlink calls, so fallback ordering is unit-testable without
  a real network/OS call.

## Platform behavior

### Linux

Query wireless interfaces via the `nl80211` crate (netlink generic socket, no
shell-out to `iw`/`nmcli`/`iwgetid`). If a wireless interface is associated to
an AP, return `Ssid`. If no wireless interface exists or it's not associated
(e.g. wired ethernet), fall through to gateway MAC.

### macOS

Query `CWInterface.ssid()` via the `objc2-core-wlan` crate. This requires the
process to hold Location Services (TCC) authorization — modern macOS returns
`nil` from `CWInterface.ssid()` without it. whichnet does not attempt to work
around this via IORegistry keys or the root-owned known-networks plist (an
approach used by third-party tools such as get-ssid-rs) — that path is fragile
(undocumented keys, needs root/setuid) and out of scope. If SSID is unavailable
(no permission, or not on Wi-Fi), fall through to gateway MAC.

### Fallback (both platforms)

Default-gateway MAC address via the `default-net` crate (maintained,
cross-platform, single API, works on Wi-Fi and wired). Subnet included as a
secondary distinguisher when available. Trade-off (accepted, carried over from
the original hivenet decision): weaker signal than exact SSID — two networks
sharing an unconfigured/default router's gateway MAC would collide — but
buildable today, testable in CI, no privileged OS calls required.

If neither SSID nor gateway MAC can be determined, return `Unknown`.

## Testing

- Fallback ordering and `IdentitySource` trait dispatch: unit-testable with a
  mock `IdentitySource` (no real OS/network access needed).
- Actual SSID retrieval (`nl80211` on Linux, CoreWLAN on macOS) and gateway MAC
  retrieval (`default-net`): not unit-testable offline — exercised via CI
  matrix (Linux + macOS runners) as integration checks, not deterministic
  asserts (network identity varies by CI runner).

## Out of scope / deferred

- Windows support (not needed for current Hive Mode scope).
- The larger "auto-pause on untrusted network" feature (trusted-network store,
  pause/resume coordinator, poll loop, dashboard CRUD UI) — separate,
  not-yet-started effort, ~6-9 tasks, sequenced after this crate and after Plan 2
  (data sync), around Plan 3 (dashboard UI).
- Workaround-based SSID retrieval on macOS that avoids the TCC prompt.
