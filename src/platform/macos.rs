use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub name: String,
    pub root: String,
    pub bin: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(installed_apps, derive(Clone))]
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
        let installed_apps: Vec<App> = Vec::new();

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
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(apps, derive(Clone))]
#[cfg_attr(apps_sys, derive(Clone))]
#[cfg_attr(apps_user, derive(Clone))]
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

        let mut apps: Vec<App> = Vec::new();

        // 应用程序安装目录
        let applications_dir = Path::new("/Applications");

        // 获取应用程序列表
        if let Ok(entries) = fs::read_dir(applications_dir) {
            for entry in entries.filter_map(Result::ok) {
                let app_path = entry.path();

                // 确保是应用程序包（以 .app 结尾）
                if app_path.extension().and_then(|e| e.to_str()) == Some("app") {
                    // 获取应用程序名称
                    let app_name = app_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");

                    // 获取 Info.plist 文件路径
                    let info_plist_path = app_path.join("Contents/Info.plist");

                    // 解析 Info.plist 文件
                    if let Ok(plist_data) = Installed::read_plist(&info_plist_path) {
                        // 应用程序名称和版本
                        let _app_version = plist_data
                            .as_dictionary()
                            .and_then(|dict| dict.get("CFBundleShortVersionString"))
                            .and_then(|v| v.as_string())
                            .unwrap_or("Unknown");

                        // 应用程序可执行文件路径
                        let executable_path = app_path.join("Contents/MacOS").join(
                            plist_data
                                .as_dictionary()
                                .and_then(|dict| dict.get("CFBundleExecutable"))
                                .and_then(|v| v.as_string())
                                .unwrap_or(""),
                        );

                        // 应用程序图标路径
                        let icon_path = app_path.join("Contents/Resources/AppIcon.icns");

                        // 输出应用程序信息
                        apps.push(App {
                            name: app_name.to_string(),
                            root: app_path.to_str().unwrap().to_string(),
                            bin: executable_path.to_str().unwrap().to_string(),
                            icon: icon_path.to_str().unwrap().to_string(),
                        });
                    }
                }
            }
        } else {
            eprintln!("Failed to read applications directory.");
        }

        Installed {
            apps,
            apps_sys,
            apps_user,
        }
    }

    // 读取并解析 plist 文件
    fn read_plist(path: &Path) -> Result<plist::Value, plist::Error> {
        let plist_path = Path::new(path);
        // 打开文件并创建一个读取器
        let file = File::open(&plist_path).expect("Failed to open plist file");
        let reader = BufReader::new(file);
        // 使用 plist crate 解析 plist 文件
        match plist::Value::from_reader(reader) {
            Ok(plist::Value::Dictionary(dict)) => {
                // 在这里处理字典类型的 plist 数据
                return Ok(plist::Value::Dictionary(dict));
            }
            Ok(other) => {
                return Ok(other);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
