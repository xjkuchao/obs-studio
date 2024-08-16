use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use anyhow::anyhow;
use configparser::ini::Ini;
use log::error;
use tauri::{AppHandle, Manager};

use crate::Result;

pub type ConfigMap = HashMap<String, HashMap<String, Option<String>>>;
pub static GLOBAL_CONFIG: OnceLock<Mutex<ConfigMap>> = OnceLock::new();

pub fn setup_global_config(app: &AppHandle) -> Result<()> {
    GLOBAL_CONFIG.get_or_init(|| {
        let mut global_config: ConfigMap = HashMap::new();
        let global_file = match app
            .path()
            .resolve("global.ini", tauri::path::BaseDirectory::AppLocalData)
        {
            Ok(global_file) => global_file,
            Err(e) => {
                error!("global.ini file not found {}", e);
                return Mutex::new(HashMap::new());
            }
        };

        let mut config = Ini::new_cs();
        let globals = match config.load(&global_file) {
            Ok(globals) => globals,
            Err(e) => {
                error!("failed to load global.ini: {}", e);
                return Mutex::new(HashMap::new());
            }
        };

        for (key, value) in globals {
            // debug!("globals: {} {:?}", key, value);

            global_config.insert(key, value.clone());
        }

        // debug!("globals: {:?}", global_config);

        Mutex::new(global_config)
    });

    Ok(())
}

pub fn get_config(section: &str, key: &str) -> Option<String> {
    match GLOBAL_CONFIG.get() {
        Some(global_config) => match global_config.lock() {
            Ok(global_config) => match global_config.get(section) {
                Some(section) => match section.get(key) {
                    Some(value) => value.clone(),
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

pub fn set_config(section: &str, key: &str, value: &str) -> Result<()> {
    match GLOBAL_CONFIG.get() {
        Some(global_config) => match global_config.lock() {
            Ok(mut global_config) => match global_config.get_mut(section) {
                Some(section) => {
                    section.insert(key.to_string(), Some(value.to_string()));
                    Ok(())
                }
                None => {
                    let mut section_value = HashMap::new();
                    section_value.insert(key.to_string(), Some(value.to_string()));
                    global_config.insert(section.to_string(), section_value);
                    Ok(())
                }
            },
            Err(e) => Err(anyhow!("failed to lock global config: {}", e)),
        },
        None => Err(anyhow!("global config not initialized")),
    }
}
