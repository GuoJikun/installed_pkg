#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod installed_pkg;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod installed_pkg;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod installed_pkg;

pub use installed_pkg::App;
