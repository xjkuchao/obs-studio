/// 系统托盘相关功能模块
use tauri::{
    menu::{Menu, MenuId, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Manager, Wry,
};

use crate::{utils::locale::t, Result, MAIN_TRAY_ID, MAIN_WINDOW_ID};

/// 设置系统托盘菜单
///
/// # 参数
///
/// * `app` - 应用程序句柄
/// * `show` - 是否显示主窗口
///
/// # 返回值
///
/// 返回 `Result<Option<Menu<Wry>>>`，表示菜单创建是否成功
fn setup_tray_menu(app: &AppHandle, show: bool) -> Result<Option<Menu<Wry>>> {
    let title = if show {
        t("Basic.SystemTray.Show")?
    } else {
        t("Basic.SystemTray.Hide")?
    };

    let tray_menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "Basic.SystemTray.Show", title, true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "Basic.SystemTray.Exit", t("Exit")?, true, None::<&str>)?,
        ],
    )?;

    Ok(Some(tray_menu))
}

/// 切换主窗口的显示状态
///
/// # 参数
///
/// * `app` - 应用程序句柄
fn toggle_main_window(app: &AppHandle) {
    let main_window = app.get_window(MAIN_WINDOW_ID).unwrap();
    let system_tray = app.tray_by_id(MAIN_TRAY_ID).unwrap();

    if main_window.is_visible().unwrap() {
        system_tray
            .set_menu(setup_tray_menu(app, true).unwrap())
            .unwrap();

        main_window.hide().unwrap();
    } else {
        system_tray
            .set_menu(setup_tray_menu(app, false).unwrap())
            .unwrap();

        main_window.show().unwrap();
        main_window.set_focus().unwrap();
    }
}

/// 设置系统托盘
///
/// # 参数
///
/// * `app` - 应用程序句柄
///
/// # 返回值
///
/// 返回 `Result<()>`，表示操作是否成功
pub fn setup_tray(app: &AppHandle) -> Result<()> {
    let system_tray = match app.tray_by_id(MAIN_TRAY_ID) {
        Some(tray) => tray,
        None => return Ok(()),
    };

    // 默认显示隐藏
    system_tray.set_menu(setup_tray_menu(app, false)?)?;

    // 设置菜单事件处理
    system_tray.on_menu_event(|app, event| {
        if event.id == MenuId::new("Basic.SystemTray.Show") {
            toggle_main_window(app);
        }

        if event.id == MenuId::new("Basic.SystemTray.Exit") {
            app.exit(0);
        }
    });

    // 设置托盘图标事件处理
    system_tray.on_tray_icon_event(|tray, event| {
        if event.id() == MAIN_TRAY_ID {
            match event {
                TrayIconEvent::Click {
                    button,
                    button_state,
                    ..
                } => match button {
                    MouseButton::Left => {
                        if button_state == MouseButtonState::Up {
                            toggle_main_window(tray.app_handle());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    });

    Ok(())
}
