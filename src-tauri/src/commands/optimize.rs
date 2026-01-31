use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::commands::files::ImageFile;
use crate::optimizer::{cwebp, mozjpeg, pngquant};
use crate::utils::file_utils::{
    get_file_size, get_file_url, get_temp_file_path, md5_string,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeOptions {
    /// 2~256, for PNG
    pub color: Option<u32>,
    /// 10~100, for JPEG/WebP
    pub quality: Option<u32>,
    /// Target format
    pub export_ext: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeRequest {
    pub image: ImageFile,
    pub options: OptimizeOptions,
}

/// Optimize an image
#[tauri::command]
pub async fn optimize_image(request: OptimizeRequest) -> Result<ImageFile, String> {
    let image = &request.image;
    let options = &request.options;

    // Determine export extension
    let export_ext = options
        .export_ext
        .clone()
        .unwrap_or_else(|| image.ext.clone());

    // Generate unique ID for optimized image
    let options_json = serde_json::to_string(&options).unwrap_or_default();
    let optimized_id = md5_string(&format!("{}{}", image.id, options_json));

    // Source and destination paths
    let source_path = get_temp_file_path(&image.id, &image.ext);
    let dest_path = get_temp_file_path(&optimized_id, &export_ext);

    // Check if already cached
    if dest_path.exists() {
        let size = get_file_size(&dest_path).map_err(|e| e.to_string())?;
        let url = get_file_url(&dest_path);

        return Ok(ImageFile {
            id: optimized_id,
            url,
            size,
            ext: export_ext,
            original_name: image.original_name.clone(),
        });
    }

    // Handle cross-format conversion: JPEG to PNG on Windows/Linux
    // pngquant on Windows/Linux doesn't support JPEG input
    let mut actual_source = source_path.clone();
    if cfg!(not(target_os = "macos")) && image.ext == "jpg" && export_ext == "png" {
        let intermediate_path = get_temp_file_path(&format!("{}_intermediate", image.id), "png");

        if !intermediate_path.exists() {
            convert_image(&source_path, &intermediate_path)?;
        }

        actual_source = intermediate_path;
    }

    // Run the appropriate optimizer
    match export_ext.as_str() {
        "png" => {
            let color = options.color.unwrap_or(256);
            pngquant::optimize(&actual_source, &dest_path, color)?;
        }
        "jpg" | "jpeg" => {
            let quality = options.quality.unwrap_or(70);
            mozjpeg::optimize(&actual_source, &dest_path, quality)?;
        }
        "webp" => {
            let quality = options.quality.unwrap_or(80);
            cwebp::optimize(&actual_source, &dest_path, quality)?;
        }
        "ico" | "bmp" | "gif" => {
            convert_image(&actual_source, &dest_path)?;
        }
        _ => {
            return Err(format!("Unsupported format: {}", export_ext));
        }
    }

    // Get result file size
    let size = get_file_size(&dest_path).map_err(|e| e.to_string())?;
    let url = get_file_url(&dest_path);

    Ok(ImageFile {
        id: optimized_id,
        url,
        size,
        ext: export_ext,
        original_name: image.original_name.clone(),
    })
}

/// Convert image format using the image crate
fn convert_image(source: &Path, dest: &Path) -> Result<(), String> {
    log::info!("Converting {:?} to {:?}", source, dest);

    let img = image::open(source).map_err(|e| format!("Failed to open image: {}", e))?;

    img.save(dest)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(())
}
