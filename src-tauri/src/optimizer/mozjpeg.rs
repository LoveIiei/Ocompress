use std::path::Path;
use super::{get_bin_path, run_optimizer};

/// Optimize JPEG using mozjpeg
pub fn optimize(
    input: &Path,
    output: &Path,
    quality: u32,
) -> Result<(), String> {
    let bin_path = get_bin_path("moz-cjpeg");

    let quality_str = quality.to_string();
    let input_str = input.to_string_lossy();
    let output_str = output.to_string_lossy();

    let args = vec![
        "-quality",
        quality_str.as_str(),
        "-outfile",
        output_str.as_ref(),
        input_str.as_ref(),
    ];

    run_optimizer(&bin_path, &args)?;
    Ok(())
}
