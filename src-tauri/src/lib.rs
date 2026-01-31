mod commands;
mod optimizer;
mod utils;

use commands::{files, optimize, placeholder, save};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    Emitter, Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            // When a second instance is launched, focus the main window and process files
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }

            // Send file paths to the frontend if any were passed
            if argv.len() > 1 {
                let file_paths: Vec<String> = argv[1..].to_vec();
                let _ = app.emit("files-dropped", file_paths);
            }
        }))
        .setup(|app| {
            // Create application menu
            let app_handle = app.handle();
            create_menu(app_handle)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event {
                let file_paths: Vec<String> = paths
                    .iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                let _ = window.emit("files-dropped", file_paths);
            }
        })
        .invoke_handler(tauri::generate_handler![
            files::process_files,
            files::open_file_dialog,
            files::clean_temp_dir,
            optimize::optimize_image,
            placeholder::generate_placeholder,
            save::save_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_menu(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let about = MenuItem::with_id(app, "about", "About Ocompress", true, None::<&str>)?;
    let quit = PredefinedMenuItem::quit(app, Some("Quit"))?;
    let separator = PredefinedMenuItem::separator(app)?;

    let app_submenu = Submenu::with_items(app, "Ocompress", true, &[&about, &separator, &quit])?;

    let open = MenuItem::with_id(app, "open", "Open", true, Some("CmdOrCtrl+O"))?;
    let save = MenuItem::with_id(app, "save", "Save", true, Some("CmdOrCtrl+S"))?;
    let save_new = MenuItem::with_id(app, "save_new", "Save with New Names", true, None::<&str>)?;
    let save_dir = MenuItem::with_id(app, "save_dir", "Save to Directory", true, None::<&str>)?;

    let file_submenu = Submenu::with_items(
        app,
        "File",
        true,
        &[&open, &separator, &save, &save_new, &save_dir],
    )?;

    let menu = Menu::with_items(app, &[&app_submenu, &file_submenu])?;

    app.set_menu(menu)?;

    // Handle menu events
    app.on_menu_event(move |app, event| {
        match event.id().as_ref() {
            "about" => {
                let _ = app.emit("menu-about", ());
            }
            "open" => {
                let _ = app.emit("menu-open", ());
            }
            "save" => {
                let _ = app.emit("menu-save", "OVER");
            }
            "save_new" => {
                let _ = app.emit("menu-save", "NEW_NAME");
            }
            "save_dir" => {
                let _ = app.emit("menu-save", "NEW_DIR");
            }
            _ => {}
        }
    });

    Ok(())
}
