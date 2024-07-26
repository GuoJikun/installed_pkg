use lnk::ShellLink;
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::reg_key::RegKey;
use winreg::HKEY;

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    pub name: String,
    pub root: String,
    pub bin: String,
    pub icon: String,
}

struct AppList {
    pub installed_apps: Vec<App>,
}

impl Iterator for AppList {
    type Item = App;

    fn next(&mut self) -> Option<Self::Item> {
        self.installed_apps.pop()
    }
}

impl AppList {
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
                return app_paths;
            },
            Err(_) => {
                return app_paths;
            },
        }
        
        
    }
    pub fn new(hive: HKEY, path: &str) -> AppList {
        let hklm = RegKey::predef(hive);
        let software_key = hklm
            .open_subkey_with_flags(
                path,
                KEY_READ,
            )
            .unwrap();
        
        let app_paths = AppList::get_apps_path(&hklm, path);
        // println!("App Paths: {:?}", app_paths);
        // Get all subkeys (each represents an installed software)
        let subkeys: Vec<String> = software_key.enum_keys().map(|x| x.unwrap()).collect();

        // HashMap to store software name and install location
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
                // println!("Root: {}", install_location);
                // println!("Bin: {}", path.display());
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
                    traverse_dir(&file_path, app_paths);
                } else {
                    // 检查文件扩展名
                    if let Some(extension) = file_path.extension() {
                        
                        if extension == "lnk" {
                            // 处理 .lnk 文件
                            match resolve_lnk_target(&file_path) {
                                Some(tmp) => {
                                    println!("binnary path: {:?}", tmp);
                                    app_paths.push(tmp);
                                },
                                None => {
                                    println!("None inside")
                                },
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
                match resolve_lnk_target(&file_path) {
                    Some(tmp) => {
                        println!("binnary path: {:?}", tmp);
                        app_paths.push(tmp);
                    },
                    None => {
                        println!("None outside")
                    },
                }

            } 
        }
    }
}
fn get_apps_path_by_startmenu() -> Vec<PathBuf> {
    let appdata = std::env::var("APPDATA").expect("Failed to get APPDATA environment variable");
    let start_menu_path = Path::new(&appdata).join("Microsoft\\Windows\\Start Menu\\Programs");
    let mut app_paths: Vec<PathBuf> = Vec::new();
    traverse_dir(&start_menu_path, &mut app_paths);
    return app_paths;
}
fn main() {
    
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


    // println!("Installed Apps Count:{}", apps.len());
    // // Print the list
    // for app in apps {
        // println!("Name: {}", app.name);
        // println!("Root: {}", app.root);
        // println!("Bin: {}", app.bin);
        // println!("Icon: {}", app.icon);
    // }

    // 遍历开始菜单目录下的所有文件
    let app_paths = get_apps_path_by_startmenu();
    for path in app_paths {
        println!("Path: {:?}", path);
    }
}
