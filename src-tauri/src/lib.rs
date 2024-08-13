use tauri::Manager;

mod cmds;
mod ui;
mod utils;

type Result<T> = anyhow::Result<T, anyhow::Error>;

#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // 初始化
    builder = builder.setup(|app| {
        utils::locale::load_locales(app.app_handle())?;
        ui::menu::setup_menus(app.app_handle())?;

        if cfg!(debug_assertions) {
            // `main` is the first window from tauri.conf.json without an explicit label
            app.get_webview_window("main").unwrap().open_devtools();
        }

        Ok(())
    });

    // 插件
    builder = builder.plugin(
        tauri_plugin_log::Builder::new()
            .clear_targets()
            .targets([
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview)
                    .filter(|metadata| !metadata.target().starts_with("webview")),
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some(chrono::Local::now().format("%Y-%m-%d").to_string()),
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
            .build(),
    );
    builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());

    // 事件
    builder = builder.invoke_handler(tauri::generate_handler![
        cmds::locale::get_default_locale,
        cmds::locale::get_locale_messages
    ]);

    // 运行
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
