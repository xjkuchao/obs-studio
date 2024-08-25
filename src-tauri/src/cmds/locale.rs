use crate::utils::locale::{LocaleMap, LOCALES};

/// 获取当前语言环境
///
/// 此函数返回当前应用程序的语言环境代码字符串。
///
/// # 返回值
///
/// - `Ok(String)`: 成功时返回当前语言环境代码字符串
/// - `Err(String)`: 如果出现错误,返回错误信息字符串
#[tauri::command]
pub fn get_locale() -> Result<String, String> {
    let locale = crate::utils::locale::get_locale().unwrap();

    Ok(locale)
}

/// 设置应用程序的语言环境
///
/// 此函数尝试将应用程序的语言环境设置为指定的值。
///
/// # 参数
///
/// * `locale` - 要设置的语言环境代码字符串
///
/// # 返回值
///
/// - `Ok(())`: 如果成功设置了语言环境
/// - `Err(String)`: 如果设置语言环境时出错,返回错误信息字符串
#[tauri::command]
pub fn set_locale(locale: &str) -> Result<(), String> {
    match crate::utils::locale::set_locale(locale) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}

/// 获取本地化消息
///
/// 此函数返回当前加载的所有本地化消息。
///
/// # 返回值
///
/// - `Ok(LocaleMap)`: 成功时返回包含所有本地化消息的 `LocaleMap`
/// - `Err(String)`: 如果出现错误,返回错误信息字符串
#[tauri::command]
pub fn get_locale_messages() -> Result<LocaleMap, String> {
    // 从全局 LOCALES 中获取本地化消息并克隆
    // unwrap() 在这里是安全的,因为 LOCALES 应该在程序启动时就已初始化
    Ok(LOCALES.get().unwrap().clone())
}
