use sys_locale::get_locale;

use crate::utils::locale::{LocaleMap, LOCALES};

#[tauri::command]
pub fn get_default_locale() -> std::result::Result<String, String> {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));

    Ok(locale)
}

#[tauri::command]
pub fn get_locale_messages() -> std::result::Result<LocaleMap, String> {
    Ok(LOCALES.get().unwrap().clone())
}
