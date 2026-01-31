use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;

/// Get the temporary directory for Imagine
pub fn get_temp_dir() -> PathBuf {
    let temp_dir = std::env::temp_dir().join("ocompress");
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).ok();
    }
    temp_dir
}

/// Clean the temporary directory
pub fn clean_temp_dir() -> Result<(), std::io::Error> {
    let temp_dir = get_temp_dir();
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
        fs::create_dir_all(&temp_dir)?;
    }
    Ok(())
}

/// Calculate MD5 hash of a string
pub fn md5_string(text: &str) -> String {
    let digest = md5::compute(text.as_bytes());
    format!("{:x}", digest)
}

/// Calculate MD5 hash of a file
pub fn md5_file(file_path: &Path) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let digest = md5::compute(&contents);
    Ok(format!("{:x}", digest))
}

/// Get file size in bytes
pub fn get_file_size(file_path: &Path) -> Result<u64, std::io::Error> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len())
}

/// Detect image type from file
pub fn detect_image_type(file_path: &Path) -> Option<String> {
    let kind = infer::get_from_path(file_path).ok()??;
    match kind.mime_type() {
        "image/png" => Some("png".to_string()),
        "image/jpeg" => Some("jpg".to_string()),
        "image/webp" => Some("webp".to_string()),
        "image/x-icon" => Some("ico".to_string()),
        "image/bmp" => Some("bmp".to_string()),
        "image/gif" => Some("gif".to_string()),
        _ => None,
    }
}

/// Get file path in temp directory for an image
pub fn get_temp_file_path(id: &str, ext: &str) -> PathBuf {
    get_temp_dir().join(format!("{}.{}", id, ext))
}

/// Get file URL from path
pub fn get_file_url(file_path: &Path) -> String {
    format!("file://{}", file_path.to_string_lossy().replace('\\', "/"))
}

/// Get an unoccupied file path by appending (1), (2), etc.
pub fn get_unoccupied_path(file_path: &Path) -> PathBuf {
    if !file_path.exists() {
        return file_path.to_path_buf();
    }

    let stem = file_path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = file_path.extension().map(|e| e.to_string_lossy().to_string());
    let parent = file_path.parent().unwrap_or(Path::new("."));

    let mut index = 1;
    loop {
        let new_name = match &ext {
            Some(e) => format!("{}({}).{}", stem, index, e),
            None => format!("{}({})", stem, index),
        };
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        index += 1;
    }
}

/// Flatten a list of files/directories into just files
pub fn flatten_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut result = Vec::new();

    for path in paths {
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.is_file() {
                result.push(path.clone());
            } else if metadata.is_dir() {
                if let Ok(entries) = fs::read_dir(path) {
                    let sub_paths: Vec<PathBuf> = entries
                        .filter_map(|e| e.ok())
                        .map(|e| e.path())
                        .collect();
                    result.extend(flatten_files(&sub_paths));
                }
            }
        }
    }

    result
}

/// Change file extension
/// 'path/to/image.png' + 'jpg' -> 'path/to/image.jpg'
pub fn reext(filename: &str, new_ext: &str) -> String {
    let path = Path::new(filename);

    if let Some(current_ext) = path.extension() {
        let current_ext_lower = current_ext.to_string_lossy().to_lowercase();
        let new_ext_lower = new_ext.to_lowercase();

        // Check if extensions match (including aliases like jpeg -> jpg)
        let matches = current_ext_lower == new_ext_lower
            || (current_ext_lower == "jpeg" && new_ext_lower == "jpg")
            || (current_ext_lower == "jpg" && new_ext_lower == "jpeg");

        if matches {
            return filename.to_string();
        }

        // Check if current extension is a supported image type
        let is_image_ext = matches!(current_ext_lower.as_str(), "png" | "jpg" | "jpeg" | "webp" | "ico" | "bmp" | "gif");

        if is_image_ext {
            // Replace extension
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let parent = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
            if parent.is_empty() {
                return format!("{}.{}", stem, new_ext);
            }
            return format!("{}/{}.{}", parent, stem, new_ext);
        }
    }

    // No extension or non-image extension: append new extension
    format!("{}.{}", filename, new_ext)
}
