//! Static build info for Siemens Nx Cad.

pub const BUILD_VERSION: &str  = "v3.1.18";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "9a97a8a014be";
pub const BUILD_TIMESTAMP: u64 = 1796663851;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Siemens Nx Cad", BUILD_VERSION, BUILD_CHANNEL)
}
