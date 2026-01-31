use std::path::Path;
use super::{get_bin_path, run_optimizer};

/// Convert to WebP using cwebp
pub fn optimize(
    input: &Path,
    output: &Path,
    quality: u32,
) -> Result<(), String> {
    let bin_path = get_bin_path("cwebp");

    let quality_str = quality.to_string();
    let input_str = input.to_string_lossy();
    let output_str = output.to_string_lossy();

    let args = vec![
        "-q",
        quality_str.as_str(),
        input_str.as_ref(),
        "-o",
        output_str.as_ref(),
    ];

    run_optimizer(&bin_path, &args)?;
    Ok(())
}
