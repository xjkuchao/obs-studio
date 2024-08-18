use tauri::Manager;

mod cmds;
mod graphics;
mod protocols;
mod ui;
mod utils;

type Result<T> = anyhow::Result<T, anyhow::Error>;

pub const MAIN_WINDOW_ID: &str = "main";
pub const MAIN_TRAY_ID: &str = "main";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    builder = builder.plugin(tauri_plugin_cli::init());

    // 初始化
    builder = builder.setup(|app| {
        utils::locale::load_locales(app.app_handle())?;

        utils::cli::setup_cli(app.app_handle())?;

        utils::config::setup_global_config(app.app_handle())?;

        ui::menu::setup_menus(app.app_handle())?;
        ui::tray::setup_tray(app.app_handle())?;

        ui::layout::setup_layout(app.app_handle())?;

        Ok(())
    });

    // 插件
    builder = builder.plugin(utils::log::setup_log().unwrap());
    builder = builder.plugin(
        tauri_plugin_window_state::Builder::default()
            .with_state_flags(
                tauri_plugin_window_state::StateFlags::default()
                    - tauri_plugin_window_state::StateFlags::VISIBLE,
            )
            .build(),
    );

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
