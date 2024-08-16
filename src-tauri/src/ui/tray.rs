use tauri::{
    menu::{Menu, MenuId, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    AppHandle, Manager, Wry,
};

use crate::{utils::trans::t, Result};

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

fn toggle_main_window(app: &AppHandle) {
    let main_window = app.get_window("main").unwrap();
    let system_tray = app.tray_by_id("main").unwrap();

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

pub fn setup_tray(app: &AppHandle) -> Result<()> {
    let system_tray = match app.tray_by_id("main") {
        Some(tray) => tray,
        None => return Ok(()),
    };

    // 默认显示隐藏
    system_tray.set_menu(setup_tray_menu(app, false)?)?;

    system_tray.on_menu_event(|app, event| {
        if event.id == MenuId::new("Basic.SystemTray.Show") {
            toggle_main_window(app);
        }

        if event.id == MenuId::new("Basic.SystemTray.Exit") {
            app.exit(0);
        }
    });

    system_tray.on_tray_icon_event(|tray, event| {
        if event.id() == "main" {
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
