// this hides the console for Windows release builds
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::Serialize;
use std::sync::Mutex;
use tauri::{
  // state is used in Linux
  self,
  Manager,
  State,
  Emitter
};
use tauri_plugin_store;
use tauri_plugin_window_state;
// use window_shadows::set_shadow;

mod sys_tray;
mod utils;

use sys_tray::{create_tray_icon, tray_update_lang, TrayState};
use utils::{long_running_thread, show_item_in_folder, show_main_window};

#[derive(Clone, Serialize)]
struct SingleInstancePayload {
  args: Vec<String>,
  cwd: String,
}

#[derive(Debug, Default, Serialize)]
struct Example<'a> {
  #[serde(rename = "Attribute 1")]
  attribute_1: &'a str,
}

#[cfg(target_os = "linux")]
pub struct DbusState(Mutex<Option<dbus::blocking::SyncConnection>>);

#[tauri::command]
fn process_file(filepath: String) -> String {
  println!("Processing file: {}", filepath);
  "Hello from Rust!".into()
}

#[cfg(target_os = "linux")]
fn webkit_hidpi_workaround() {
  // See: https://github.com/spacedriveapp/spacedrive/issues/1512#issuecomment-1758550164
  std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
}

fn main_prelude() {
  #[cfg(target_os = "linux")]
  webkit_hidpi_workaround();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  main_prelude();
  // main window should be invisible to allow either the setup delay or the plugin to show the window
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_notification::init())
    // custom commands
    .invoke_handler(tauri::generate_handler![
      tray_update_lang,
      process_file,
      show_item_in_folder
    ])
    // allow only one instance and propagate args and cwd to existing instance
    .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
      app
        .emit("newInstance", SingleInstancePayload { args, cwd })
        .unwrap();
    }))
    // persistent storage with filesystem
    .plugin(tauri_plugin_store::Builder::default().build())
    // save window position and size between sessions
    // if you remove this, make sure to uncomment the show_main_window code
    //  in this file and TauriProvider.jsx
    .plugin(tauri_plugin_window_state::Builder::default().build())
    // .manage(Mutex::new(TrayState::NotPlaying))
    // custom setup code
    .setup(|app| {
      app.manage(Mutex::new(TrayState::NotPlaying));
      if let Some(window) = app.get_webview_window("main") {
        window.set_shadow(true).ok(); // Ignore errors if platform is unsupported
      }

      #[cfg(target_os = "linux")]
      app.manage(DbusState(Mutex::new(
        dbus::blocking::SyncConnection::new_session().ok(),
      )));
      let app_handle = app.handle().clone();
      tauri::async_runtime::spawn(async move { long_running_thread(&app_handle).await });

      // Create the tray icon
      create_tray_icon(app.app_handle())?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// useful crates
// https://crates.io/crates/directories for getting common directories

// TODO: optimize permissions
// TODO: decorations false and use custom title bar
