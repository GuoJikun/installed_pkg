pub mod platform;
pub use platform::Installed;

pub fn list() -> Result<Installed, String> {
    Ok(Installed::new())
}
