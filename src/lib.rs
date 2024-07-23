mod platform;
pub use platform::App;

pub fn list() -> Result<Vec<App>, String> {
    App::new()
}
