use std::path::Path;
use super::{get_bin_path, run_optimizer};

/// Optimize PNG using pngquant
pub fn optimize(
    input: &Path,
    output: &Path,
    color: u32,
) -> Result<(), String> {
    let bin_path = get_bin_path("pngquant");

    let color_str = color.to_string();
    let input_str = input.to_string_lossy();
    let output_str = output.to_string_lossy();

    let args = vec![
        color_str.as_str(),
        input_str.as_ref(),
        "-o",
        output_str.as_ref(),
    ];

    run_optimizer(&bin_path, &args)?;
    Ok(())
}
