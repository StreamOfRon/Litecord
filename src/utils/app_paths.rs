//! Cross-platform per-user config directory for Litecord's persisted state
//! (localization-consistency design: OS-standard dirs, not app-local ones).

use std::path::PathBuf;

/// OS-standard per-user config directory:
/// `%APPDATA%\Litecord` on Windows, `~/Library/Application Support/Litecord`
/// on macOS, `$XDG_CONFIG_HOME/litecord` (default `~/.config/litecord`) on
/// other unix. Creates nothing — callers run `create_dir_all` before writing.
#[must_use]
pub(crate) fn config_dir() -> PathBuf {
    let fallback = PathBuf::from(".");
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::var_os("USERPROFILE")
                    .map(|h| PathBuf::from(h).join("AppData").join("Roaming"))
                    .unwrap_or_else(|| fallback)
            });
        base.join("Litecord")
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| fallback)
            .join("Library")
            .join("Application Support")
            .join("Litecord")
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::var_os("HOME")
                    .map(|h| PathBuf::from(h).join(".config"))
                    .unwrap_or_else(|| fallback)
            })
            .join("litecord")
    }
}
