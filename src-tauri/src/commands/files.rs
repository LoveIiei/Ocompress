use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri_plugin_dialog::DialogExt;

use crate::utils::file_utils::{
    detect_image_type, flatten_files, get_file_url, get_temp_file_path, md5_file, md5_string,
    get_file_size, clean_temp_dir as clean_temp,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageFile {
    pub id: String,
    pub url: String,
    pub size: u64,
    pub ext: String,
    pub original_name: String,
}

/// Process files dropped or selected - copy to temp dir and return metadata
#[tauri::command]
pub async fn process_files(file_paths: Vec<String>) -> Result<Vec<ImageFile>, String> {
    let paths: Vec<PathBuf> = file_paths.iter().map(PathBuf::from).collect();
    let flat_files = flatten_files(&paths);

    let mut results = Vec::new();

    for file_path in flat_files {
        // Detect image type
        let ext = match detect_image_type(&file_path) {
            Some(e) => e,
            None => continue, // Skip unsupported files
        };

        // Calculate unique ID from path + content hash
        let path_str = file_path.to_string_lossy().to_string();
        let path_hash = md5_string(&path_str);
        let content_hash = md5_file(&file_path).map_err(|e| e.to_string())?;
        let id = format!("{}{}", path_hash, content_hash);

        // Get file size
        let size = get_file_size(&file_path).map_err(|e| e.to_string())?;

        // Copy to temp directory
        let temp_path = get_temp_file_path(&id, &ext);

        // Only copy if not already exists
        if !temp_path.exists() {
            fs::copy(&file_path, &temp_path).map_err(|e| e.to_string())?;
        }

        let url = get_file_url(&temp_path);

        results.push(ImageFile {
            id,
            url,
            size,
            ext,
            original_name: path_str,
        });
    }

    Ok(results)
}

/// Open file dialog and process selected files
#[tauri::command]
pub async fn open_file_dialog(app: tauri::AppHandle) -> Result<Vec<ImageFile>, String> {
    let file_paths = app
        .dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg"])
        .set_title("Choose images")
        .blocking_pick_files();

    match file_paths {
        Some(paths) => {
            let path_strings: Vec<String> = paths
                .iter()
                .filter_map(|p| p.as_path().map(|path| path.to_string_lossy().to_string()))
                .collect();
            process_files(path_strings).await
        }
        None => Ok(Vec::new()),
    }
}

/// Clean the temp directory
#[tauri::command]
pub async fn clean_temp_dir() -> Result<(), String> {
    clean_temp().map_err(|e| e.to_string())
}
