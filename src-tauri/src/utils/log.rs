/// 日志模块
use tauri::Wry;

use crate::Result;

/// 调试模式下的日志级别
#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

/// 非调试模式下的日志级别
#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

/// 设置日志系统
///
/// # 返回值
///
/// 返回 `Result<tauri::plugin::TauriPlugin<Wry>>`，表示日志插件的配置结果
pub fn setup_log() -> Result<tauri::plugin::TauriPlugin<Wry>> {
    let log_plugin = tauri_plugin_log::Builder::new()
        .clear_targets()
        .targets([
            // 配置Webview目标
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview).filter(
                |metadata| {
                    let target = metadata.target();

                    // 过滤掉特定的目标
                    !(target.starts_with("webview")
                        || target.starts_with("log@")
                        || target.starts_with("wgpu_core::")
                        || target.starts_with("naga::")
                        || target.starts_with("wgpu_hal::"))
                },
            ),
            // 配置主日志文件目标
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                file_name: Some(chrono::Local::now().format("main_%Y-%m-%d").to_string()),
            })
            .filter(|metadata| {
                let target = metadata.target();
                // 过滤掉特定的目标
                !(target.starts_with("wgpu_core::")
                    || target.starts_with("naga::")
                    || target.starts_with("wgpu_hal::"))
            }),
            // 配置WGPU日志文件目标
            tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                file_name: Some(chrono::Local::now().format("wgpu_%Y-%m-%d").to_string()),
            })
            .filter(|metadata| {
                let target = metadata.target();
                // 只包含特定的目标，并限制日志级别
                (target.starts_with("wgpu_core::")
                    || target.starts_with("naga::")
                    || target.starts_with("wgpu_hal::"))
                    && metadata.level() <= log::Level::Info
            }),
        ])
        .level(LOG_LEVEL)
        .format(|out, message, record| {
            // 自定义日志格式
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
