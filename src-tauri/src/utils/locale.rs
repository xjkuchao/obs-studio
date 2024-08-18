use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use anyhow::anyhow;
use ini::Ini;
use tauri::{AppHandle, Manager};

use crate::utils::dialog::message;
use crate::Result;

pub type LocaleMap = HashMap<String, HashMap<String, Option<String>>>;
pub static LOCALES: OnceLock<LocaleMap> = OnceLock::new();

static CURRENT_LOCALE: OnceLock<Mutex<String>> = OnceLock::new();

pub fn load_locales(app: &AppHandle) -> Result<()> {
    LOCALES.get_or_init(|| {
        let mut locale_messages: LocaleMap = HashMap::new();
        let locales_file = match app
            .path()
            .resolve("resources/locale.ini", tauri::path::BaseDirectory::Resource)
        {
            Ok(locales_file) => locales_file,
            Err(e) => {
                message(
                    app,
                    "error",
                    "Error",
                    format!("resources/locale.ini {}", e).as_str(),
                    1,
                );
                panic!("locale.ini file not found {}", e);
            }
        };

        let config = match Ini::load_from_file(&locales_file) {
            Ok(config) => config,
            Err(e) => {
                message(
                    app,
                    "error",
                    "Error",
                    format!("resources/locale.ini {}", e).as_str(),
                    1,
                );
                panic!("failed to load locale.ini: {}", e);
            }
        };

        config.sections().all(|section| {
            if section.is_some() {
                let local_file_name =
                    PathBuf::from("resources/locale").join(section.unwrap().to_string() + ".ini");
                let locale_file = match app.path().resolve(
                    local_file_name.clone(),
                    tauri::path::BaseDirectory::Resource,
                ) {
                    Ok(locale_file) => locale_file,
                    Err(e) => {
                        message(
                            app,
                            "error",
                            "Error",
                            format!("{} {}", local_file_name.display(), e).as_str(),
                            1,
                        );
                        panic!(
                            "locale file not found: {}: {}",
                            local_file_name.display(),
                            e
                        );
                    }
                };

                let local_config = match Ini::load_from_file(&locale_file) {
                    Ok(local_config) => local_config,
                    Err(e) => {
                        message(
                            app,
                            "error",
                            "Error",
                            format!("{} {}", local_file_name.display(), e).as_str(),
                            1,
                        );
                        panic!(
                            "failed to load locale file: {}: {}",
                            local_file_name.display(),
                            e
                        );
                    }
                };

                let mut messages = HashMap::new();
                local_config.general_section().iter().all(|(key, value)| {
                    let key = key.to_string();
                    let value = value.trim_matches('"').to_string();

                    messages.insert(key, Some(value));

                    true
                });
                locale_messages.insert(section.unwrap().to_string(), messages);
            }

            true
        });

        locale_messages
    });

    Ok(())
}

pub fn get_locale() -> Result<String> {
    let current_locale = CURRENT_LOCALE.get_or_init(|| {
        let mut locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));

        // MacOS uses zh-Hans-CN, but we need zh-CN
        if locale == "zh-Hans-CN" {
            locale = "zh-CN".to_string();
        }

        Mutex::new(locale)
    });

    Ok(current_locale.lock().unwrap().clone())
}

pub fn set_locale(locale: &str) -> Result<()> {
    // locale must in the supported locales
    let support_locales = LOCALES
        .get()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<String>>();
    if !support_locales.contains(&locale.to_string()) {
        return Err(anyhow!("Unsupported locale: {}", locale));
    }

    let mut current_locale = CURRENT_LOCALE.get().unwrap().lock().unwrap();
    *current_locale = locale.to_string();

    Ok(())
}

pub fn t(key: &str) -> Result<String> {
    let locale = get_locale()?;

    let locales = match LOCALES.get() {
        Some(locales) => locales,
        None => return Ok(key.to_string()),
    };

    let locale = match locales.get(&locale) {
        Some(locale) => locale,
        None => return Ok(key.to_string()),
    };

    // let messages = match locale.get("translation") {
    //     Some(messages) => messages,
    //     None => return Ok(key.to_string()),
    // };

    match locale.get(key) {
        Some(message) => Ok(message.clone().unwrap_or(key.to_string())),
        None => Ok(key.to_string()),
    }
}
