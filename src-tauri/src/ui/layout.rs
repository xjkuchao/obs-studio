/// 此模块处理应用程序的布局和窗口管理
use std::{collections::HashMap, sync::Mutex};

use anyhow::Ok;
use lazy_static::lazy_static;
use tauri::{
    webview::WebviewBuilder, window::WindowBuilder, AppHandle, LogicalPosition, LogicalSize,
    Manager, RunEvent, Theme, WebviewUrl,
};

use crate::{
    graphics::context::Context,
    utils::{cli::cli, config::get_config, locale::t},
    Result, MAIN_WINDOW_ID,
};

/// 存储窗口上下文的全局静态变量
lazy_static! {
    static ref WINDOW_CONTEXT: Mutex<HashMap<String, Context>> = Mutex::new(HashMap::new());
}

/// 处理布局相关的事件
///
/// # 参数
///
/// * `app` - 应用程序句柄
/// * `event` - 运行时事件
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作是否成功
pub fn layout_event(app: &AppHandle, event: &RunEvent) -> Result<()> {
    match event {
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if let Some(context) = WINDOW_CONTEXT.lock().unwrap().get_mut(label) {
                match event {
                    tauri::WindowEvent::Resized(size) => {
                        context.resize(size);
                    }
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        if cli()?.opt_minimize_tray {
                            let window = app.get_window(label).unwrap();

                            api.prevent_close();
                            window.hide()?;
                        }
                    }

                    _ => {}
                }
            }
        }
        tauri::RunEvent::MainEventsCleared => {
            if let Some(context) = WINDOW_CONTEXT.lock().unwrap().get(MAIN_WINDOW_ID) {
                context.render();
            }
        }
        _ => {}
    }

    Ok(())
}

/// 设置应用程序的布局
///
/// # 参数
///
/// * `app` - 应用程序句柄
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作是否成功
pub fn setup_layout(app: &AppHandle) -> Result<()> {
    let width = 1086.;
    let height = 729.;

    #[cfg(not(target_os = "macos"))]
    let window = WindowBuilder::new(app, MAIN_WINDOW_ID)
        .inner_size(width, height)
        .theme(Some(Theme::Dark))
        .visible(false)
        .build()?;
    #[cfg(target_os = "macos")]
    let window = WindowBuilder::new(app, MAIN_WINDOW_ID)
        .inner_size(width, height)
        .theme(Some(Theme::Dark))
        .build()?;

    let cli = cli()?;
    if cli.opt_always_on_top {
        window.set_always_on_top(true)?;
    }

    WINDOW_CONTEXT
        .lock()
        .unwrap()
        .insert(window.label().to_string(), Context::new(app, &window)?);

    let size: LogicalSize<f64> = window.inner_size()?.to_logical(window.scale_factor()?);
    let total_width = size.width;
    let total_height = size.height;

    // preview 窗口高度占比
    let preview_height = 0.64;

    let docks_x = 0.;
    let docks_y = total_height * preview_height;
    let docks_width = total_width;
    let docks_height = total_height * (1.0 - preview_height);
    let docks = window.add_child(
        WebviewBuilder::new("docks", WebviewUrl::App(Default::default())).auto_resize(),
        LogicalPosition::new(docks_x, docks_y),
        LogicalSize::new(docks_width, docks_height),
    )?;

    #[cfg(debug_assertions)]
    docks.open_devtools();

    update_title(app)?;

    Ok(())
}

/// 更新应用程序窗口的标题
///
/// # 参数
///
/// * `app` - 应用程序句柄
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作是否成功
pub fn update_title(app: &AppHandle) -> Result<()> {
    let window = app.get_window(MAIN_WINDOW_ID).unwrap();

    let profile = get_config("Basic", "Profile");
    let scene_collection = get_config("Basic", "SceneCollection");

    let mut title = "OBS ".to_string();
    title += &app.package_info().version.to_string();

    title += " - ";
    title += t("TitleBar.Profile")?.as_str();
    title += ": ";
    title += &profile.unwrap_or("".to_string());

    title += " - ";
    title += t("TitleBar.Scenes")?.as_str();
    title += ": ";
    title += &scene_collection.unwrap_or("".to_string());

    window.set_title(&title)?;

    Ok(())
}
