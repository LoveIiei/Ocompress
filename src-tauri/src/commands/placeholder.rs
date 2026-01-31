use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use log::info;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderRequest {
    pub width: u32,
    pub height: u32,
    pub background_color: String,
    pub text_color: String,
    pub text: Option<String>,
    pub format: String,
}

/// Parse hex color string to RGB values
fn parse_hex_color(hex: &str) -> Result<Rgb<u8>, String> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(format!("Invalid hex color length: {} (expected 6)", hex.len()));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|e| format!("Invalid red component: {}", e))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|e| format!("Invalid green component: {}", e))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|e| format!("Invalid blue component: {}", e))?;

    Ok(Rgb([r, g, b]))
}

/// Embedded font data
const FONT_DATA: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf");

/// Generate a placeholder image
#[tauri::command]
pub async fn generate_placeholder(
    app: tauri::AppHandle,
    request: PlaceholderRequest,
) -> Result<(), String> {
    info!(
        "generate_placeholder: {}x{}, bg={}, text_color={}, format={}",
        request.width, request.height, request.background_color, request.text_color, request.format
    );

    // Validate and clamp dimensions
    let width = request.width.max(1).min(4096);
    let height = request.height.max(1).min(4096);

    // Parse colors
    let bg_color = parse_hex_color(&request.background_color)
        .map_err(|e| format!("Background color error: {}", e))?;
    let text_color = parse_hex_color(&request.text_color)
        .map_err(|e| format!("Text color error: {}", e))?;

    info!("Colors parsed: bg={:?}, text={:?}", bg_color, text_color);

    // Create image with background color
    let mut img: RgbImage = ImageBuffer::from_pixel(width, height, bg_color);
    info!("Image buffer created: {}x{}", width, height);

    // Determine text to display
    let display_text = match &request.text {
        Some(t) if !t.is_empty() => t.clone(),
        _ => format!("{}x{}", width, height),
    };
    info!("Display text: {}", display_text);

    // Load font
    let font = FontRef::try_from_slice(FONT_DATA)
        .map_err(|e| format!("Font loading error: {}", e))?;
    info!("Font loaded successfully");

    // Calculate font size based on image dimensions
    let mut font_size = (height as f32 * 0.15).max(12.0).min(200.0);

    // Simple text width estimation and adjustment
    let char_count = display_text.len() as f32;
    let max_text_width = width as f32 * 0.8;
    let estimated_width = char_count * font_size * 0.6;

    if estimated_width > max_text_width {
        font_size = (max_text_width / char_count / 0.6).max(12.0);
    }

    let scale = PxScale::from(font_size);

    // Calculate text position (centered)
    let text_width = (char_count * font_size * 0.6) as i32;
    let text_height = font_size as i32;
    let x = ((width as i32 - text_width) / 2).max(0);
    let y = ((height as i32 - text_height) / 2).max(0);

    info!("Drawing text at ({}, {}) with size {}", x, y, font_size);

    // Draw text
    draw_text_mut(&mut img, text_color, x, y, scale, &font, &display_text);
    info!("Text drawn successfully");

    // Determine format and extension
    let (img_format, ext) = match request.format.to_lowercase().as_str() {
        "jpg" | "jpeg" => (ImageFormat::Jpeg, "jpg"),
        "webp" => (ImageFormat::WebP, "webp"),
        _ => (ImageFormat::Png, "png"),
    };
    info!("Output format: {:?}", img_format);

    // Default filename
    let default_name = format!("placeholder_{}x{}.{}", width, height, ext);

    // Show save dialog
    info!("Showing save dialog...");
    let file_path = app
        .dialog()
        .file()
        .set_title("Save Placeholder Image")
        .set_file_name(&default_name)
        .add_filter("Image", &[ext])
        .blocking_save_file();

    let file_path = match file_path {
        Some(p) => {
            info!("Save dialog returned: {:?}", p);
            match p.as_path() {
                Some(path) => path.to_path_buf(),
                None => {
                    info!("Path conversion returned None");
                    return Ok(());
                }
            }
        }
        None => {
            info!("User cancelled save dialog");
            return Ok(());
        }
    };

    info!("Saving to: {:?}", file_path);

    // Encode image to bytes
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, img_format)
        .map_err(|e| format!("Image encoding error: {}", e))?;
    info!("Image encoded successfully, {} bytes", buffer.get_ref().len());

    // Write to file
    std::fs::write(&file_path, buffer.into_inner())
        .map_err(|e| format!("File write error: {}", e))?;

    info!("Placeholder saved successfully to {:?}", file_path);
    Ok(())
}
