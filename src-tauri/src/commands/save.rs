use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::ShellExt;

use crate::commands::files::ImageFile;
use crate::utils::file_utils::{get_temp_file_path, get_unoccupied_path, reext};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SaveType {
    Over,
    NewName,
    NewDir,
    SaveAs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveRequest {
    pub images: Vec<ImageFile>,
    pub save_type: SaveType,
}

/// Save files based on save type
#[tauri::command]
pub async fn save_files(app: tauri::AppHandle, request: SaveRequest) -> Result<(), String> {
    let images = &request.images;
    let save_type = &request.save_type;

    match save_type {
        SaveType::Over => save_over(images).await,
        SaveType::NewName => save_new_name(images).await,
        SaveType::NewDir => save_to_new_dir(&app, images).await,
        SaveType::SaveAs => save_as(&app, images).await,
    }
}

/// Save by overwriting original files
async fn save_over(images: &[ImageFile]) -> Result<(), String> {
    for image in images {
        let source_path = get_temp_file_path(&image.id, &image.ext);
        let dest_path = reext(&image.original_name, &image.ext);

        copy_file(&source_path, Path::new(&dest_path))?;
    }
    Ok(())
}

/// Save with new names (appending index)
async fn save_new_name(images: &[ImageFile]) -> Result<(), String> {
    for image in images {
        let source_path = get_temp_file_path(&image.id, &image.ext);
        let dest_path_str = reext(&image.original_name, &image.ext);
        let dest_path = get_unoccupied_path(Path::new(&dest_path_str));

        copy_file(&source_path, &dest_path)?;
    }
    Ok(())
}

/// Save to a new directory
async fn save_to_new_dir(app: &tauri::AppHandle, images: &[ImageFile]) -> Result<(), String> {
    // Show directory picker
    let dir_path = app
        .dialog()
        .file()
        .set_title("Save files")
        .blocking_pick_folder();

    let dir_path = match dir_path {
        Some(p) => match p.as_path() {
            Some(path) => path.to_path_buf(),
            None => return Ok(()),
        },
        None => return Ok(()), // User cancelled
    };

    for image in images {
        let source_path = get_temp_file_path(&image.id, &image.ext);
        let base_name = Path::new(&image.original_name)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let dest_name = reext(&base_name, &image.ext);
        let dest_path = dir_path.join(&dest_name);
        let final_path = get_unoccupied_path(&dest_path);

        copy_file(&source_path, &final_path)?;
    }

    // Open the directory
    #[allow(deprecated)]
    let _ = app.shell().open(dir_path.to_string_lossy().as_ref(), None);

    Ok(())
}

/// Save as (single file with dialog)
async fn save_as(app: &tauri::AppHandle, images: &[ImageFile]) -> Result<(), String> {
    if images.is_empty() {
        return Ok(());
    }

    let image = &images[0];
    let default_name = Path::new(&image.original_name)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let default_name = reext(&default_name, &image.ext);

    // Show save dialog
    let file_path = app
        .dialog()
        .file()
        .set_title("Save file")
        .set_file_name(&default_name)
        .add_filter("Images", &[&image.ext])
        .blocking_save_file();

    let file_path = match file_path {
        Some(p) => match p.as_path() {
            Some(path) => path.to_path_buf(),
            None => return Ok(()),
        },
        None => return Ok(()), // User cancelled
    };

    let source_path = get_temp_file_path(&image.id, &image.ext);
    copy_file(&source_path, &file_path)?;

    Ok(())
}

/// Copy file helper
fn copy_file(source: &Path, dest: &Path) -> Result<(), String> {
    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::copy(source, dest)
        .map_err(|e| format!("Failed to copy file: {}", e))?;

    Ok(())
}
