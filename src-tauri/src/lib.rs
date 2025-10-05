#![cfg_attr(mobile, allow(unused))]

mod dependencies;

use anyhow::Result;
use dependencies::{check_dependencies, install_dependencies};
use std::path::Path;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_autostart::MacosLauncher;

#[tauri::command]
fn run_job(job: cadinho_core::Job) -> Result<(), String> {
    if !Path::new(&job.input).exists() {
        return Err(format!("Entrada não existe: {}", job.input));
    }
    if let Some(parent) = Path::new(&job.output).parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Criando pasta de saída: {e}"))?;
    }
    cadinho_core::convert(&job).map_err(|e| format!("{e:#}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // log
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(|out, message, record| {
                    out.finish(format_args!("[{}] {}", record.level(), message))
                })
                .build(),
        )
        // dialog
        .plugin(tauri_plugin_dialog::init())
        // single instance
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .setup(|app| {
            // autostart
            #[cfg(desktop)]
            let _ = app.handle().plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec!["--flag1", "--flag2"]),
            ));

            // menu bandeja
            let quit = MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;
            let toggle = MenuItem::with_id(app, "toggle", "Mostrar/Esconder", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&toggle, &quit])?;

            // tray
            TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.unminimize();
                                let _ = w.set_focus();
                            }
                        }
                    }
                })
                .on_menu_event(|app, e| match e.id.as_ref() {
                    "toggle" => {
                        if let Some(w) = app.get_webview_window("main") {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            run_job,
            check_dependencies,
            install_dependencies
        ])
        .run(tauri::generate_context!())
        .expect("falha ao iniciar Cadinho");
}
