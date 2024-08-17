use std::{collections::HashMap, sync::Mutex};

use anyhow::Ok;
use lazy_static::lazy_static;
use tauri::{
    webview::WebviewBuilder, window::WindowBuilder, AppHandle, LogicalPosition, LogicalSize,
    Manager, RunEvent, WebviewUrl,
};

use crate::{
    graphics::context::Context,
    utils::{config::get_config, trans::t},
    Result,
};

lazy_static! {
    static ref WINDOW_CONTEXT: Mutex<HashMap<String, Context>> = Mutex::new(HashMap::new());
}

pub fn layout_event(_app: &AppHandle, event: &RunEvent) -> Result<()> {
    match event {
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if let Some(context) = WINDOW_CONTEXT.lock().unwrap().get_mut(label) {
                match event {
                    tauri::WindowEvent::Resized(size) => {
                        context.resize(size);
                    }

                    _ => {}
                }
            }
        }
        tauri::RunEvent::MainEventsCleared => {
            if let Some(context) = WINDOW_CONTEXT.lock().unwrap().get("main") {
                context.render();
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn setup_layout(app: &AppHandle) -> Result<()> {
    // let window = app.get_window("main").unwrap();
    let width = 1086.;
    let height = 729.;
    let window = WindowBuilder::new(app, "main")
        .inner_size(width, height)
        .build()?;

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

    // `main` is the first window from tauri.conf.json without an explicit label
    #[cfg(debug_assertions)]
    docks.open_devtools();

    update_title(app)?;

    Ok(())
}

pub fn update_title(app: &AppHandle) -> Result<()> {
    let window = app.get_window("main").unwrap();

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
