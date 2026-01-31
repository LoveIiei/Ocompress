pub mod pngquant;
pub mod mozjpeg;
pub mod cwebp;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;

/// Get the base path for optimizer binaries
pub fn get_bin_base_path() -> PathBuf {
    // In development, binaries are in src-tauri/bin
    // In production, they are in the resources directory
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(Path::new("."));

    // Check for production path first (resources/bin)
    let resource_bin = exe_dir.join("bin");
    if resource_bin.exists() {
        return resource_bin;
    }

    // Check for development path
    let dev_bin = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bin");
    if dev_bin.exists() {
        return dev_bin;
    }

    // Fallback
    resource_bin
}

/// Get the platform-specific binary path
pub fn get_bin_path(name: &str) -> PathBuf {
    let base = get_bin_base_path();

    let platform = if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    };

    let arch = if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x64"
    };

    let binary_name = if cfg!(target_os = "windows") {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };

    base.join(platform).join(arch).join(binary_name)
}

/// Run an optimizer command
pub fn run_optimizer(
    bin_path: &Path,
    args: &[&str],
) -> Result<std::process::Output, String> {
    log::info!("Running optimizer: {:?} {:?}", bin_path, args);

    let mut cmd = Command::new(bin_path);
    cmd.args(args);

    // Set LD_LIBRARY_PATH for Linux
    if cfg!(target_os = "linux") {
        let lib_path = bin_path.parent().unwrap_or(Path::new("."));
        cmd.env("LD_LIBRARY_PATH", lib_path);
    }

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(output)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                Err(format!(
                    "Command failed with exit code {:?}\nstdout: {}\nstderr: {}",
                    output.status.code(),
                    stdout,
                    stderr
                ))
            }
        }
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}
