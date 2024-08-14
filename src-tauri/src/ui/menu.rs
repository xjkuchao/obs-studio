use log::debug;
use tauri::{
    AppHandle,
    Manager, menu::{
        CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu,
    }, Wry,
};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

use crate::Result;
use crate::utils::trans::t;

pub fn setup_menus(app: &AppHandle) -> Result<()> {
    let menu = Menu::with_items(
        app,
        &[
            &Submenu::with_id_and_items(
                app,
                "Basic.MainMenu.File",
                t("Basic.MainMenu.File")?,
                true,
                &[
                    &MenuItem::with_id(
                        app,
                        "Basic.MainMenu.File.ShowRecordings",
                        t("Basic.MainMenu.File.ShowRecordings")?,
                        true,
                        None::<&str>,
                    )?,
                    &MenuItem::with_id(
                        app,
                        "Basic.MainMenu.File.Remux",
                        t("Basic.MainMenu.File.Remux")?,
                        true,
                        None::<&str>,
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(
                        app,
                        "Basic.MainMenu.File.Settings",
                        t("Basic.MainMenu.File.Settings")?,
                        true,
                        None::<&str>,
                    )?,
                    &MenuItem::with_id(
                        app,
                        "Basic.MainMenu.File.ShowSettingsFolder",
                        t("Basic.MainMenu.File.ShowSettingsFolder")?,
                        true,
                        None::<&str>,
                    )?,
                    &MenuItem::with_id(
                        app,
                        "Basic.MainMenu.File.ShowProfileFolder",
                        t("Basic.MainMenu.File.ShowProfileFolder")?,
                        true,
                        None::<&str>,
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(
                        app,
                        "Basic.MainMenu.File.Exit",
                        t("Basic.MainMenu.File.Exit")?,
                        true,
                        None::<&str>,
                    )?,
                ],
            )?,
            &Submenu::with_id_and_items(
                app,
                "Basic.MainMenu.Edit",
                t("Basic.MainMenu.Edit")?,
                true,
                &[
                    &MenuItem::with_id(app, "Undo.Undo", t("Undo.Undo")?, false, None::<&str>)?,
                    &MenuItem::with_id(app, "Undo.Redo", t("Undo.Redo")?, false, None::<&str>)?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(app, "Copy", t("Copy")?, true, None::<&str>)?,
                    &MenuItem::with_id(
                        app,
                        "PasteReference",
                        t("PasteReference")?,
                        false,
                        None::<&str>,
                    )?,
                    &MenuItem::with_id(
                        app,
                        "PasteDuplicate",
                        t("PasteDuplicate")?,
                        true,
                        None::<&str>,
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(
                        app,
                        "Copy.Filters",
                        t("Copy.Filters")?,
                        true,
                        None::<&str>,
                    )?,
                    &MenuItem::with_id(
                        app,
                        "Paste.Filters",
                        t("Paste.Filters")?,
                        false,
                        None::<&str>,
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &Submenu::with_id_and_items(
                        app,
                        "Basic.MainMenu.Edit.Transform",
                        t("Basic.MainMenu.Edit.Transform")?,
                        true,
                        &[
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.EditTransform",
                                t("Basic.MainMenu.Edit.Transform.EditTransform")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.CopyTransform",
                                t("Basic.MainMenu.Edit.Transform.CopyTransform")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.PasteTransform",
                                t("Basic.MainMenu.Edit.Transform.PasteTransform")?,
                                false,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.ResetTransform",
                                t("Basic.MainMenu.Edit.Transform.ResetTransform")?,
                                true,
                                None::<&str>,
                            )?,
                            &PredefinedMenuItem::separator(app)?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.Rotate90CW",
                                t("Basic.MainMenu.Edit.Transform.Rotate90CW")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.Rotate90CCW",
                                t("Basic.MainMenu.Edit.Transform.Rotate90CCW")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.Rotate180",
                                t("Basic.MainMenu.Edit.Transform.Rotate180")?,
                                true,
                                None::<&str>,
                            )?,
                            &PredefinedMenuItem::separator(app)?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.FlipHorizontal",
                                t("Basic.MainMenu.Edit.Transform.FlipHorizontal")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.FlipVertical",
                                t("Basic.MainMenu.Edit.Transform.FlipVertical")?,
                                true,
                                None::<&str>,
                            )?,
                            &PredefinedMenuItem::separator(app)?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.FitToScreen",
                                t("Basic.MainMenu.Edit.Transform.FitToScreen")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.StretchToScreen",
                                t("Basic.MainMenu.Edit.Transform.StretchToScreen")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.CenterToScreen",
                                t("Basic.MainMenu.Edit.Transform.CenterToScreen")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.VerticalCenter",
                                t("Basic.MainMenu.Edit.Transform.VerticalCenter")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Transform.HorizontalCenter",
                                t("Basic.MainMenu.Edit.Transform.HorizontalCenter")?,
                                true,
                                None::<&str>,
                            )?,
                        ],
                    )?,
                    &Submenu::with_id_and_items(
                        app,
                        "Basic.MainMenu.Edit.Order",
                        t("Basic.MainMenu.Edit.Order")?,
                        true,
                        &[
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Order.MoveUp",
                                t("Basic.MainMenu.Edit.Order.MoveUp")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Order.MoveDown",
                                t("Basic.MainMenu.Edit.Order.MoveDown")?,
                                true,
                                None::<&str>,
                            )?,
                            &PredefinedMenuItem::separator(app)?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Order.MoveToTop",
                                t("Basic.MainMenu.Edit.Order.MoveToTop")?,
                                true,
                                None::<&str>,
                            )?,
                            &MenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Order.MoveToBottom",
                                t("Basic.MainMenu.Edit.Order.MoveToBottom")?,
                                true,
                                None::<&str>,
                            )?,
                        ],
                    )?,
                    &Submenu::with_id_and_items(
                        app,
                        "Basic.MainMenu.Edit.Scale",
                        t("Basic.MainMenu.Edit.Scale")?,
                        true,
                        &[
                            &CheckMenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Scale.Window",
                                t("Basic.MainMenu.Edit.Scale.Window")?,
                                true,
                                true,
                                None::<&str>,
                            )?,
                            &CheckMenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Scale.Canvas",
                                t("Basic.MainMenu.Edit.Scale.Canvas")?,
                                true,
                                false,
                                None::<&str>,
                            )?,
                            &CheckMenuItem::with_id(
                                app,
                                "Basic.MainMenu.Edit.Scale.Output",
                                t("Basic.MainMenu.Edit.Scale.Output")?,
                                true,
                                false,
                                None::<&str>,
                            )?,
                        ],
                    )?,
                ],
            )?,
        ],
    )?;

    app.set_menu(menu)?;

    app.on_menu_event(|app, event| {
        if event.id == MenuId::new("Basic.MainMenu.Edit.Scale.Window") {
            let menu = app
                .menu()
                .unwrap()
                .get("Basic.MainMenu.Edit")
                .unwrap()
                .as_submenu()
                .unwrap()
                .get("Basic.MainMenu.Edit.Scale")
                .unwrap()
                .as_submenu()
                .unwrap()
                .get("Basic.MainMenu.Edit.Scale.Window")
                .unwrap()
                .as_check_menuitem()
                .unwrap()
                .clone();

            let is_checked = menu.is_checked().unwrap();

            debug!("is_checked: {:?} {:?}", menu.id(), is_checked);
        }

        if event.id == MenuId::new("Basic.MainMenu.File.Exit") {
            app.exit(0);
        }
    });

    Ok(())
}

fn setup_tray_menu(app: &AppHandle, show: bool) -> Result<Option<Menu<Wry>>> {
    let title = if show {
        t("Basic.SystemTray.Show")?
    } else {
        t("Basic.SystemTray.Hide")?
    };

    let tray_menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(
                app,
                "Basic.SystemTray.Show",
                title,
                true,
                None::<&str>,
            )?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                "Basic.SystemTray.Exit",
                t("Exit")?,
                true,
                None::<&str>,
            )?,
        ],
    )?;

    Ok(Some(tray_menu))
}

fn toggle_main_window(app: &AppHandle) {
    let main_window = app.get_webview_window("main").unwrap();
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
