use crate::utils::locale::{get_locale, LocaleMap, LOCALES};

#[tauri::command]
pub fn get_default_locale() -> Result<String, String> {
    let locale = get_locale().unwrap();

    Ok(locale)
}

#[tauri::command]
pub fn get_locale_messages() -> Result<LocaleMap, String> {
    Ok(LOCALES.get().unwrap().clone())
}
