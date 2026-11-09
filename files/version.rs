//! Static build info for Addictive Keys.

pub const BUILD_VERSION: &str  = "v4.5.5";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "27af5f3aaf3f";
pub const BUILD_TIMESTAMP: u64 = 1708739927;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Addictive Keys", BUILD_VERSION, BUILD_CHANNEL)
}
