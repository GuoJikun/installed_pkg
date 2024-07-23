pub struct App {
    pub name: String,
    pub version: String,
    pub publisher: String,
}

struct AppList {
    pub installed_apps: Vec<App>,
}
impl AppList {
    pub fn new() -> Self {
        let mut installed_apps: Vec<App> = Vec::new();

        AppList { installed_apps }
    }
}

impl App {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn publisher(&self) -> &str {
        &self.publisher
    }

    pub fn new() -> Result<Vec<App>, String> {
        let mut apps: Vec<App> = Vec::new();
        Ok(apps)
    }
}
