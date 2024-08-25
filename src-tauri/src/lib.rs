/// 使用 tauri::Manager 特性
use tauri::Manager;

/// 导入模块
mod cmds;
mod graphics;
mod protocols;
mod ui;
mod utils;

/// 定义 Result 类型别名
type Result<T> = anyhow::Result<T, anyhow::Error>;

/// 定义主窗口和主托盘的常量 ID
pub const MAIN_WINDOW_ID: &str = "main";
pub const MAIN_TRAY_ID: &str = "main";

/// 应用程序入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    /// 创建 Tauri 应用程序构建器
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_dialog::init());

    /// 添加命令行界面插件
    builder = builder.plugin(tauri_plugin_cli::init());

    /// 设置应用程序初始化
    builder = builder.setup(|app| {
        /// 加载本地化资源
        utils::locale::load_locales(app.app_handle())?;

        /// 设置命令行界面
        utils::cli::setup_cli(app.app_handle())?;

        /// 设置全局配置
        utils::config::setup_global_config(app.app_handle())?;

        /// 设置菜单
        ui::menu::setup_menus(app.app_handle())?;
        /// 设置系统托盘
        ui::tray::setup_tray(app.app_handle())?;

        /// 设置布局
        ui::layout::setup_layout(app.app_handle())?;

        Ok(())
    });

    /// 添加日志插件
    builder = builder.plugin(utils::log::setup_log().unwrap());
    /// 添加窗口状态插件
    builder = builder.plugin(
        tauri_plugin_window_state::Builder::default()
            .with_state_flags(
                tauri_plugin_window_state::StateFlags::default()
                    - tauri_plugin_window_state::StateFlags::VISIBLE,
            )
            .build(),
    );

    /// 注册自定义命令处理器
    builder = builder.invoke_handler(tauri::generate_handler![
        cmds::locale::get_locale,
        cmds::locale::set_locale,
        cmds::locale::get_locale_messages
    ]);

    /// 构建并运行 Tauri 应用程序
    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    /// 运行应用程序并处理布局事件
    app.run(|app, event| {
        ui::layout::layout_event(app, &event).expect("layout event error");
    });
}
