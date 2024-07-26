use std::fs;
use std::path::{Path, PathBuf};

use lnk::ShellLink;
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::reg_key::RegKey;
use winreg::HKEY;

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn new(hive: HKEY, path: &str) -> AppList {
        let hklm = RegKey::predef(hive);
        let software_key = hklm
            .open_subkey_with_flags(
                path,
                KEY_READ,
            )
            .unwrap();
        
        let app_paths = AppList::get_apps_path(&hklm, path);

        // Get all subkeys (each represents an installed software)
        let subkeys: Vec<String> = software_key.enum_keys().map(|x| x.unwrap()).collect();

        // 初始化已安装软件列表
        let mut installed_apps: Vec<App> = Vec::new();

        // Iterate over each subkey to get name and install location
        for key in subkeys {
            
            let app_key = software_key.open_subkey(&key).unwrap();
            let name: String = app_key
                .get_value("DisplayName")
                .unwrap_or_else(|_| String::from(""));
            let install_location: String = app_key
                .get_value("InstallLocation")
                .unwrap_or_else(|_| String::from(""));

            let icon: String = app_key
                .get_value("DisplayIcon")
                .unwrap_or_else(|_| String::from(""));


            if name.is_empty() || install_location.is_empty() {
                continue;
            }

            let mut bin = String::from("");
             for path in &app_paths {
                let tmp_path = path.display().to_string();
                if tmp_path.contains(&install_location) {
                    bin = tmp_path;
                    break;
                }
            }
            
            // Add software to the list
            installed_apps.push(App {
                name,
                root: install_location,
                bin,
                icon: if icon.ends_with(".ico")  {icon} else {String::from("")}
            });
        }

        AppList { installed_apps }
    }
    fn get_apps_path (hklm: &RegKey, path: &str)-> Vec<PathBuf> {
        let mut app_paths: Vec<PathBuf> = Vec::new();
        match hklm.open_subkey_with_flags(
                    path.replace("Uninstall", "App Paths"),
                    KEY_READ,
                ) {
            Ok(app_paths_key) => {
                // Iterate over subkeys (each representing an installed application)
                let subkeys: Vec<String> = app_paths_key
                    .enum_keys()
                    .map(|x| x.unwrap())
                    .collect();

                for key in subkeys {
                    if let Ok(app_key) = app_paths_key.open_subkey(&key) {
                        
                        if let Ok(path) = app_key.get_raw_value("") {
                            app_paths.push(PathBuf::from(path.to_string()));
                        }
                    }
                }
            },
            Err(_) => {
                // println!("App Paths not found");
            },
        }
        // todo: 从开始菜单拿到快捷方式对应的可执行程序路径
        let startmenu_app_bins = AppList::get_apps_path_by_startmenu();
        return app_paths.into_iter().chain(startmenu_app_bins).collect();
    }
    /// 解析 .lnk 文件的目标路径
    fn resolve_lnk_target(lnk_path: &Path) -> Option<PathBuf> {
        // 过滤掉 Windows PowerShell 的快捷方式, 等待 lnk crate 修复
        if lnk_path.display().to_string().contains("Windows PowerShell") {
            return None;
        }
        // 使用 ShellLink:: 读取快捷方式
        match ShellLink::open(lnk_path) {
            Ok(shell_link) => {
                match shell_link.link_info() {
                    Some(link_info) => {
                        return link_info.local_base_path().clone().map(PathBuf::from);
                    },
                    _ => {
                        return None;
                    },
                }
            },
            Err(_) => {
                return None
            },
        };
    }
    fn traverse_dir(dir_path: &Path, app_paths: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_dir() {
                        // 如果是目录，则递归调用
                        AppList::traverse_dir(&file_path, app_paths);
                    } else {
                        // 检查文件扩展名
                        if let Some(extension) = file_path.extension() {
                            if extension == "lnk" {
                                // 处理 .lnk 文件
                                match AppList::resolve_lnk_target(&file_path) {
                                    Some(tmp) => {
                                        app_paths.push(tmp);
                                    },
                                    None => { },
                                }
                            } 
                        }
                    }
                }
            }
        } else {
            let entry = fs::read_link(dir_path).expect("Failed to read link");
            let file_path = entry.as_path();
            // 检查文件扩展名
            if let Some(extension) = file_path.extension() {      
                if extension == "lnk" {
                    // 处理 .lnk 文件
                    match AppList::resolve_lnk_target(&file_path) {
                        Some(tmp) => {
                            app_paths.push(tmp);
                        },
                        None => { },
                    }
                } 
            }
        }
    }
    fn get_apps_path_by_startmenu() -> Vec<PathBuf> {
        let appdata = std::env::var("APPDATA").expect("Failed to get APPDATA environment variable");
        let start_menu_path = Path::new(&appdata).join("Microsoft\\Windows\\Start Menu\\Programs");
        let mut app_paths: Vec<PathBuf> = Vec::new();
        AppList::traverse_dir(&start_menu_path, &mut app_paths);
        return app_paths;
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
        let apps_sys: Vec<App> = Vec::new().into_iter().chain(system_apps.installed_apps).chain(system_apps_32.installed_apps).collect();
        
        let apps_user: Vec<App> = Vec::new().into_iter().chain(user_apps.installed_apps).chain(user_apps_32.installed_apps).collect();
        
        let apps: Vec<App> = Vec::new().into_iter().chain(apps_sys.clone()).chain(apps_user.clone()).collect();
        
        Installed {
            apps,
            apps_sys,
            apps_user,
        }
    }
}