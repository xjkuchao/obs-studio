use tauri::Manager;

mod cmds;
mod ui;
mod utils;

use ui::menu::setup_menus;
use utils::locale::load_locales;
use utils::log::setup_logger;

use cmds::locale::{get_default_locale, get_locale_messages};

type Result<T> = anyhow::Result<T, anyhow::Error>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            setup_logger(app.app_handle())?;
            load_locales(app.app_handle())?;
            setup_menus(app.app_handle())?;

            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_default_locale,
            get_locale_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
