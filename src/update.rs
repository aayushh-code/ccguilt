use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;
use std::process::Command;

const GITEA_API: &str = "http://192.168.100.195/api/v1/repos/aayush/ccguilt/releases";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn self_update() -> Result<()> {
    eprintln!(
        "  {} Checking for updates (current: v{})...",
        ">>".yellow().bold(),
        CURRENT_VERSION
    );

    // Fetch latest release from Gitea API using curl
    let output = Command::new("curl")
        .args(["-s", "-L", GITEA_API])
        .output()
        .map_err(|_| anyhow!("curl not found. Install curl to use --increase-guilt."))?;

    if !output.status.success() {
        return Err(anyhow!("Failed to reach update server."));
    }

    let releases: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)
        .map_err(|_| anyhow!("Failed to parse release info."))?;

    let latest = releases
        .first()
        .ok_or_else(|| anyhow!("No releases found."))?;

    let tag = latest["tag_name"]
        .as_str()
        .ok_or_else(|| anyhow!("Invalid release format."))?;
    let latest_version = tag.strip_prefix('v').unwrap_or(tag);

    if latest_version == CURRENT_VERSION {
        eprintln!(
            "  {} Already on latest version (v{}).",
            ">>".green().bold(),
            CURRENT_VERSION
        );
        return Ok(());
    }

    eprintln!(
        "  {} New version available: v{} -> v{}",
        ">>".yellow().bold(),
        CURRENT_VERSION,
        latest_version
    );

    // Find the right asset for this platform
    let asset_name = format!(
        "ccguilt-{}-{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let download_url = latest["assets"]
        .as_array()
        .and_then(|assets| {
            assets.iter().find_map(|a| {
                let name = a["name"].as_str()?;
                if name.contains(std::env::consts::OS) || name.contains(std::env::consts::ARCH) {
                    // Gitea browser_download_url may use unresolvable hostname, build from API
                    Some(format!(
                        "http://192.168.100.195/aayush/ccguilt/releases/download/{}/{}",
                        tag, name
                    ))
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| anyhow!("No binary found for {} in release {}.", asset_name, tag))?;

    eprintln!("  {} Downloading {}...", ">>".yellow().bold(), download_url);

    // Download to temp file
    let current_exe = std::env::current_exe()?;
    let tmp_path = current_exe.with_extension("update");

    let dl = Command::new("curl")
        .args(["-s", "-L", "-o", &tmp_path.to_string_lossy(), &download_url])
        .status()
        .map_err(|_| anyhow!("Download failed."))?;

    if !dl.success() {
        let _ = fs::remove_file(&tmp_path);
        return Err(anyhow!("Download failed."));
    }

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&tmp_path, fs::Permissions::from_mode(0o755))?;
    }

    // Replace current binary
    let backup_path = current_exe.with_extension("old");
    let _ = fs::remove_file(&backup_path);
    fs::rename(&current_exe, &backup_path)?;
    fs::rename(&tmp_path, &current_exe)?;
    let _ = fs::remove_file(&backup_path);

    eprintln!(
        "  {} Updated to v{}! The planet weeps louder.",
        ">>".green().bold(),
        latest_version
    );

    Ok(())
}
