use std::collections::HashMap;
use std::path::PathBuf;

use chrono::Local;
use configparser::ini::Ini;
use log::error;
use std::sync::OnceLock;
use sys_locale::get_locale;
use tauri::Manager;

type Result<T> = anyhow::Result<T, anyhow::Error>;
type LocaleMap = HashMap<String, HashMap<String, HashMap<String, Option<String>>>>;

const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
static LOCALES: OnceLock<LocaleMap> = OnceLock::new();

fn setup_logger(app: &tauri::App) -> Result<()> {
    let logs = app.path().app_log_dir()?;
    if !logs.exists() {
        std::fs::create_dir_all(&logs)?;
    }

    println!("logs: {}", logs.display());

    let _ = fern::Dispatch::new()
        .filter(|metadata| metadata.level() <= LOG_LEVEL)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date} {level} {target}] {message}",
                date = Local::now().format("%m.%d %H:%M:%S"),
                level = record.level(),
                target = record.target(),
                message = message,
            ))
        })
        // .level(log::LevelFilter::from_str(settings.log_level.as_str())?)
        .chain(std::io::stdout())
        .chain(fern::DateBased::new(logs, "/%Y.%m.%d.log"))
        .apply()?;

    Ok(())
}

fn load_locales(app: &tauri::App) -> Result<()> {
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

#[tauri::command]
fn get_default_locale() -> std::result::Result<String, String> {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));

    Ok(locale)
}

#[tauri::command]
fn get_locale_messages() -> std::result::Result<LocaleMap, String> {
    Ok(LOCALES.get().unwrap().clone())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            setup_logger(app)?;
            load_locales(app)?;

            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_default_locale,
            get_locale_messages,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
