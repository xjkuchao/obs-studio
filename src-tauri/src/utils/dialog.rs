use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

/// 显示一个消息对话框，并根据需要退出程序
///
/// # 参数
/// * `app` - Tauri应用程序句柄
/// * `kind` - 对话框类型（"info", "warn", "error"）
/// * `title` - 对话框标题
/// * `message` - 对话框消息内容
/// * `exit_code` - 退出代码，如果非零则退出程序
pub fn message(app: &AppHandle, kind: &str, title: &str, message: &str, exit_code: i32) {
    // 根据输入的字符串确定对话框类型
    let kind = match kind.to_lowercase().as_str() {
        "info" => MessageDialogKind::Info,
        "warn" => MessageDialogKind::Warning,
        "error" => MessageDialogKind::Error,
        _ => MessageDialogKind::Info, // 默认为信息类型
    };

    // 显示对话框
    app.dialog()
        .message(message)
        .title(title)
        .kind(kind)
        .blocking_show();

    // 如果exit_code不为0，则退出程序
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
