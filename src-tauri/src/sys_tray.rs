use serde::Serialize;
use std::sync::Mutex;
use tauri::{
  Manager, AppHandle, Emitter, Runtime,
  menu::{Menu, Submenu, MenuItem, PredefinedMenuItem, MenuBuilder, MenuItemBuilder, MenuItemKind, SubmenuBuilder},
  tray::{TrayIconBuilder, TrayIconEvent, TrayIcon},
};

#[derive(Clone, Serialize)]
pub struct SystemTrayPayload {
  message: String,
}

impl SystemTrayPayload {
  pub fn new(message: &str) -> SystemTrayPayload {
    SystemTrayPayload {
      message: message.into(),
    }
  }
}

pub enum TrayState {
  NotPlaying,
  Paused,
  Playing,
}

pub fn create_tray_menu<R: Runtime>(app: &AppHandle<R>, lang: String) -> tauri::Result<Menu<R>> {
  // TODO: tray internationalization https://docs.rs/rust-i18n/latest/rust_i18n/
  // untested, not sure if the macro accepts dynamic languages
  // ENTER rust_i18n::set_locale(lang) IF LOCAL=lang DOES NOT COMPILE
  // .add_item("id".to_string(), t!("Label", locale = lang))
  // .add_item("id".to_string(), t!("Label")

  // Create the submenu
  let submenu = SubmenuBuilder::new(app, "Sub Menu!")
    .text("bf-sep", "Before Separator")
    .separator()
    .text("af-sep", "After Separator")
    .build()?;

      // Build the main menu
  let menu = MenuBuilder::new(app)
  .item(&submenu)
  .text("quit", "Quit")
  .text("toggle-visibility", "Hide Window")
  .text("toggle-tray-icon", "Toggle the tray icon")
  .build()?;

  Ok(menu)
}

pub fn create_system_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<TrayIconBuilder<R>> {
  Ok(
    TrayIconBuilder::with_id("main-tray")
      .menu(&create_tray_menu(app, "en".into())?)
  )
}

#[tauri::command]
#[allow(unused_must_use)]
pub fn tray_update_lang(app: tauri::AppHandle, lang: String) {
  let tray_handle = app.tray_by_id("main-tray").unwrap();
  let new_menu = create_tray_menu(&app, lang);
  tray_handle.set_menu(new_menu.ok()).unwrap();
}

pub fn create_tray_icon(app: &tauri::AppHandle) -> tauri::Result<()> {
  let tray_menu = create_tray_menu(&app, "en".into());
  let tray = TrayIconBuilder::with_id("main-tray")
    // .with_id("main-tray")
    .menu(&tray_menu?)
    .on_menu_event(|app, event| {
      let id = event.id();
      // let app = tray.app_handle();
      // let item_handle = app.tray_by_id("main-tray").get_item(&id);
      if let Some (main_window) = app.get_webview_window("main") {
        main_window
          .emit("systemTray", SystemTrayPayload::new(&id.as_ref()))
          .unwrap();
      
        match id.as_ref() {
          "quit" => {
            std::process::exit(0);
          }
          "toggle-tray-icon" => {
            let tray_state_mutex = app.state::<Mutex<TrayState>>();
            let mut tray_state = tray_state_mutex.lock().unwrap();
            let tray_handle = app.tray_by_id("main-tray").unwrap();
            match *tray_state {
              TrayState::NotPlaying => {
                tray_handle
                  .set_icon(Some(tauri::image::Image::from_bytes(include_bytes!("../icons/SystemTray2.ico")).unwrap()))
                  .unwrap();
                *tray_state = TrayState::Playing;
              }
              TrayState::Playing => {
                tray_handle
                  .set_icon(Some(tauri::image::Image::from_bytes(
                    include_bytes!("../icons/SystemTray1.ico")).unwrap()
                  ))
                  .unwrap();
                *tray_state = TrayState::NotPlaying;
              }
              TrayState::Paused => {}
            };
          }
          "toggle-visibility" => {
            if main_window.is_visible().unwrap() {
              main_window.hide().unwrap();
              // item_handle.set_title("Show Window").unwrap();
            } else {
              main_window.show().unwrap();
              // item_handle.set_title("Hide Window").unwrap();
            }
          }
          _ => {}
        }
      }
    })
    .on_tray_icon_event(|trap, event| match event {
      TrayIconEvent::Click { .. } => {
        let app = trap.app_handle();
        let main_window = app.get_webview_window("main").unwrap();
        main_window
          .emit("system-tray", SystemTrayPayload::new("left-click"))
          .unwrap();
        println!("system tray received a left click");
      }
      // TrayIconEvent::RightClick { .. } => {
      //   println!("system tray received a right click");
      // }
      TrayIconEvent::DoubleClick { .. } => {
        println!("system tray received a double click");
      }
      _ => {}
    })
    .build(app)?;
  Ok(())
}
