use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub root: String,
    pub bin: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppList {
    pub installed_apps: Vec<App>,
}

impl Default for AppList {
    fn default() -> Self {
        Self {
            installed_apps: Vec::new(),
        }
    }
    
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

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn bin(&self) -> &str {
        &self.bin
    }

    pub fn icon(&self) -> &str {
        &self.icon
    }

}
#[derive(Serialize, Deserialize, Clone)]
pub struct Installed {
    pub apps: Vec<App>,
    pub apps_sys: Vec<App>,
    pub apps_user: Vec<App>,
}
impl Default for Installed {
    fn default() -> Self {
        Self {
            apps: Vec::new(),
            apps_sys: Vec::new(),
            apps_user: Vec::new(),
        }
    }
}
impl Installed {
    pub fn new() -> Self {
        let apps_sys: Vec<App> = Vec::new();
        
        let apps_user: Vec<App> = Vec::new();
        
        let apps: Vec<App> = Vec::new();
        
        Installed {
            apps,
            apps_sys,
            apps_user,
        }
    }
}