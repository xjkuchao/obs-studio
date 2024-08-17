use tauri::Wry;

use crate::Result;

#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

pub fn setup_log() -> Result<tauri::plugin::TauriPlugin<Wry>> {
    let log_plugin = tauri_plugin_log::Builder::new()
        .clear_targets()
        .targets([
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview).filter(
                |metadata| {
                    let target = metadata.target();
                    !(target.starts_with("webview")
                        || target.starts_with("wgpu_core::")
                        || target.starts_with("naga::")
                        || target.starts_with("wgpu_hal::"))
                },
            ),
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                file_name: Some(chrono::Local::now().format("main_%Y-%m-%d").to_string()),
            })
            .filter(|metadata| {
                let target = metadata.target();

                !(target.starts_with("wgpu_core::")
                    || target.starts_with("naga::")
                    || target.starts_with("wgpu_hal::"))
            }),
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                file_name: Some(chrono::Local::now().format("wgpu_%Y-%m-%d").to_string()),
            })
            .filter(|metadata| {
                let target = metadata.target();
                (target.starts_with("wgpu_core::")
                    || target.starts_with("naga::")
                    || target.starts_with("wgpu_hal::"))
                    && metadata.level() <= log::Level::Info
            }),
        ])
        .level(LOG_LEVEL)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{date} {level} {target}] {message}",
                date = chrono::Local::now().format("%m.%d %H:%M:%S"),
                level = record.level(),
                target = record.target(),
                message = message,
            ))
        })
        .build();

    Ok(log_plugin)
}
