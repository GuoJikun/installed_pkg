use serde::{Serialize, Deserialize};
use winreg::enums::*;
use winreg::reg_key::RegKey;
use winreg::HKEY;

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    pub name: String,
    pub version: String,
    pub publisher: String,
}

struct AppList {
    pub installed_apps: Vec<App>,
}

impl AppList {
    pub fn new(hive: HKEY, path: &str) -> AppList {
        let key = RegKey::predef(hive).open_subkey(path).unwrap();
        let mut installed_apps: Vec<App> = Vec::new();

        for subkey in key.enum_keys().map(|x| x.unwrap()) {
            let app_key = key.open_subkey(subkey).unwrap();
            let name: String = app_key.get_value("DisplayName").unwrap();
            let version: String = app_key.get_value("DisplayVersion").unwrap();
            let publisher: String = app_key.get_value("Publisher").unwrap();
            if name.is_empty() {
                continue;
            }
            installed_apps.push(App {
                name,
                version,
                publisher,
            });
        }

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
        let system_apps = AppList::new(
            HKEY_LOCAL_MACHINE,
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        );

        let system_apps_32 = AppList::new(
            HKEY_LOCAL_MACHINE,
            "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        );

        let user_apps = AppList::new(
            HKEY_CURRENT_USER,
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        );

        let user_apps_32 = AppList::new(
            HKEY_CURRENT_USER,
            "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        );

        let mut apps: Vec<App> = Vec::new();
        apps.extend(system_apps.installed_apps);
        apps.extend(system_apps_32.installed_apps);
        apps.extend(user_apps.installed_apps);
        apps.extend(user_apps_32.installed_apps);

        Ok(apps)
    }
}
