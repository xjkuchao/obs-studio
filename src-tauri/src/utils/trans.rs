use crate::Result;

use crate::utils::locale::{get_locale, LOCALES};

pub fn t(key: &str) -> Result<String> {
    let locale = get_locale()?;

    let locales = LOCALES.get().unwrap();
    let locale = locales.get(&locale).unwrap();
    let messages = locale.get("translation").unwrap();
    let message = messages.get(key).unwrap();

    Ok(message.clone().unwrap_or_default())
}
