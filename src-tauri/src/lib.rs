use tauri::Manager;

mod cmds;
mod graphics;
mod protocols;
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
        utils::config::setup_global_config(app.app_handle())?;
        ui::menu::setup_menus(app.app_handle())?;
        ui::tray::setup_tray(app.app_handle())?;
        ui::layout::setup_layout(app.app_handle())?;

        Ok(())
    });

    // 插件
    builder = builder.plugin(
        tauri_plugin_log::Builder::new()
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
            .build(),
    );
    builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());

    // 自定义协议
    // builder = builder.register_asynchronous_uri_scheme_protocol(
    //     "stream",
    //     move |_app, request, responder| match protocols::stream::setup_stream_protocol(request) {
    //         Ok(http_response) => responder.respond(http_response),
    //         Err(e) => responder.respond(
    //             http::response::Builder::new()
    //                 .status(http::status::StatusCode::INTERNAL_SERVER_ERROR)
    //                 .header(http::header::CONTENT_TYPE, "text/plain")
    //                 .body(e.to_string().as_bytes().to_vec())
    //                 .unwrap(),
    //         ),
    //     },
    // );

    // 事件
    builder = builder.invoke_handler(tauri::generate_handler![
        cmds::locale::get_default_locale,
        cmds::locale::get_locale_messages
    ]);

    // 运行
    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app, event| {
        ui::layout::layout_event(app, &event).expect("layout event error");
    });
}
