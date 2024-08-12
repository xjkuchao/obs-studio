use std::collections::HashMap;
use std::path::PathBuf;

use configparser::ini::Ini;
use log::error;
use std::sync::{Mutex, OnceLock};
use tauri::AppHandle;
use tauri::Manager;

use crate::Result;

pub type LocaleMap = HashMap<String, HashMap<String, HashMap<String, Option<String>>>>;

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
                error!("locale.ini file not found {}", e);
                return HashMap::new();
            }
        };

        let mut config = Ini::new_cs();
        let settings = match config.load(&locales_file) {
            Ok(settings) => settings,
            Err(e) => {
                error!("failed to load locale.ini: {}", e);
                return HashMap::new();
            }
        };

        for (key, _setting) in settings {
            // debug!("setting: {} {:?}", key, setting);

            let local_file_name = PathBuf::from("resources/locale").join(key.clone() + ".ini");
            let locale_file = match app.path().resolve(
                local_file_name.clone(),
                tauri::path::BaseDirectory::Resource,
            ) {
                Ok(locale_file) => locale_file,
                Err(e) => {
                    error!(
                        "locale file not found: {}: {}",
                        local_file_name.display(),
                        e
                    );
                    return HashMap::new();
                }
            };

            // debug!("locale file: {}", locale_file.display());

            let mut config = Ini::new_cs();
            let local_message = match config.load(&locale_file) {
                Ok(local_message) => local_message,
                Err(e) => {
                    error!(
                        "failed to load locale file: {}: {}",
                        local_file_name.display(),
                        e
                    );
                    return HashMap::new();
                }
            };

            if !local_message.contains_key("default") {
                error!(
                    "locale file does not contain default section: {}",
                    local_file_name.display()
                );
                return HashMap::new();
            }

            let messages: HashMap<String, Option<String>> = local_message["default"]
                .clone()
                .into_iter()
                .filter_map(|(k, v)| {
                    if v.is_none() {
                        Some((k.clone(), Some("".to_string())))
                    } else {
                        Some((k.clone(), Some(v.unwrap().trim_matches('"').to_string())))
                    }
                })
                .collect();

            // debug!("messages: {} {:?}", key, messages);

            let mut namespace: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();
            namespace.insert("translation".to_string(), messages);
            locale_messages.insert(key, namespace);
        }

        locale_messages
    });

    Ok(())
}

pub fn get_locale() -> Result<String> {
    let current_locale = CURRENT_LOCALE.get_or_init(|| {
        let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
        Mutex::new(locale)
    });

    Ok(current_locale.lock().unwrap().clone())
}

pub fn set_locale(locale: &str) -> Result<()> {
    let mut current_locale = CURRENT_LOCALE.get().unwrap().lock().unwrap();
    *current_locale = locale.to_string();

    Ok(())
}
