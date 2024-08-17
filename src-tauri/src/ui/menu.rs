use std::{collections::HashMap, sync::OnceLock};

use log::debug;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuId, MenuItem, MenuItemKind, PredefinedMenuItem, Submenu},
    AppHandle, Wry,
};

use crate::{utils::locale::t, Result};

type MenuMap = HashMap<MenuId, MenuItemKind<Wry>>;
static MENUS: OnceLock<MenuMap> = OnceLock::new();

fn flat_submenu(menuitem: &MenuItemKind<Wry>, menus: &mut MenuMap) {
    menus.insert(menuitem.id().clone(), menuitem.clone());

    let menu = match menuitem.as_submenu() {
        Some(submenu) => submenu,
        None => return,
    };

    let items = match menu.items() {
        Ok(items) => items,
        Err(_) => return,
    };

    for item in items {
        flat_submenu(&item, menus);
    }
}

fn flat_menu(menu: &Menu<Wry>) -> MenuMap {
    let mut menus: MenuMap = HashMap::new();

    let items = match menu.items() {
        Ok(items) => items,
        Err(_) => return menus,
    };

    for item in items {
        flat_submenu(&item, &mut menus);
    }

    menus
}

fn find_menu_by_id(app: &AppHandle, id: &MenuId) -> Option<MenuItemKind<Wry>> {
    let menus = MENUS.get_or_init(|| {
        let menu = app.menu().unwrap();
        flat_menu(&menu)
    });

    menus.get(id).cloned()
}

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
            let menu = find_menu_by_id(app, &event.id).unwrap();
            let is_checked = menu.as_check_menuitem().unwrap().is_checked().unwrap();

            debug!("is_checked: {:?} {:?}", menu.id(), is_checked);
        }

        if event.id == MenuId::new("Basic.MainMenu.File.Exit") {
            app.exit(0);
        }
    });

    Ok(())
}
