/// 全局配置模块
use std::{
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use anyhow::anyhow;
use ini::Ini;
use log::error;
use tauri::{AppHandle, Manager};

use crate::{utils::dialog::message, Result};

/// 全局配置文件路径
static GLOBAL_FILE: OnceLock<PathBuf> = OnceLock::new();
/// 全局配置对象
static GLOBAL_CONFIG: OnceLock<Mutex<Ini>> = OnceLock::new();

/// 设置全局配置
///
/// # 参数
///
/// * `app` - 应用程序句柄
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作是否成功
pub fn setup_global_config(app: &AppHandle) -> Result<()> {
    GLOBAL_FILE.get_or_init(|| {
        match app
            .path()
            .resolve("global.ini", tauri::path::BaseDirectory::AppLocalData)
        {
            Ok(global_file) => global_file,
            Err(e) => {
                message(
                    app,
                    "error",
                    "Error",
                    format!("global.ini {}", e).as_str(),
                    1,
                );
                panic!("global.ini file path not found {}", e);
            }
        }
    });

    GLOBAL_CONFIG.get_or_init(|| {
        let config = match Ini::load_from_file(GLOBAL_FILE.get().unwrap()) {
            Ok(config) => config,
            Err(_) => Ini::new(),
        };

        // debug!("globals: {:?}", config);

        Mutex::new(config)
    });

    Ok(())
}

/// 获取配置项的值
///
/// # 参数
///
/// * `section` - 配置节名
/// * `key` - 配置项名
///
/// # 返回值
///
/// 返回 `Option<String>`，表示配置项的值
pub fn get_config(section: &str, key: &str) -> Option<String> {
    match GLOBAL_CONFIG.get() {
        Some(mutex) => match mutex.lock() {
            Ok(config) => match config.section(Some(section)) {
                Some(section) => match section.get(key) {
                    Some(value) => Some(value.to_string()),
                    None => None,
                },
                None => None,
            },
            Err(e) => {
                error!("failed to lock global config: {}", e);
                None
            }
        },
        None => None,
    }
}

/// 设置配置项的值
///
/// # 参数
///
/// * `section` - 配置节名
/// * `key` - 配置项名
/// * `value` - 配置项的值
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作是否成功
pub fn set_config(section: &str, key: &str, value: &str) -> Result<()> {
    match GLOBAL_CONFIG.get() {
        Some(mutex) => match mutex.lock() {
            Ok(mut config) => {
                config
                    .with_section(Some(section))
                    .set(key, value.to_string());
                match config.write_to_file(&GLOBAL_FILE.get().unwrap()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow!("failed to write global config: {}", e)),
                }
            }
            Err(e) => Err(anyhow!("failed to lock global config: {}", e)),
        },
        None => Err(anyhow!("global config not initialized")),
    }
}
