#[cfg(target_os = "macos")]
pub mod mac_os;
#[cfg(target_os = "macos")]
pub use self::mac_os::clean_cache;
