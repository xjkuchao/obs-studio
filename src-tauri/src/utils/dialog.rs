use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

pub fn message(app: &AppHandle, kind: &str, title: &str, message: &str, exit_code: i32) {
    let kind = match kind.to_lowercase().as_str() {
        "info" => MessageDialogKind::Info,
        "warn" => MessageDialogKind::Warning,
        "error" => MessageDialogKind::Error,
        _ => MessageDialogKind::Info,
    };

    app.dialog()
        .message(message)
        .title(title)
        .kind(kind)
        .blocking_show();
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
