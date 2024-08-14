use crate::{
    utils::locale::{get_locale, LOCALES},
    Result,
};

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

    let messages = match locale.get("translation") {
        Some(messages) => messages,
        None => return Ok(key.to_string()),
    };

    match messages.get(key) {
        Some(message) => Ok(message.clone().unwrap_or(key.to_string())),
        None => Ok(key.to_string()),
    }
}
