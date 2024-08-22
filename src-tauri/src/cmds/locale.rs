use crate::utils::locale::{LocaleMap, LOCALES};

#[tauri::command]
pub fn get_locale() -> Result<String, String> {
    let locale = crate::utils::locale::get_locale().unwrap();

    Ok(locale)
}

#[tauri::command]
pub fn set_locale(locale: &str) -> Result<(), String> {
    match crate::utils::locale::set_locale(locale) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_locale_messages() -> Result<LocaleMap, String> {
    Ok(LOCALES.get().unwrap().clone())
}
