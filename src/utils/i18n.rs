use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Auto,
    English,
    Portuguese,
    Spanish,
    German,
    French,
    Russian,
    Japanese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::Auto => "auto",
            Language::English => "en",
            Language::Portuguese => "pt",
            Language::Spanish => "es",
            Language::German => "de",
            Language::French => "fr",
            Language::Russian => "ru",
            Language::Japanese => "ja",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "en" | "english" => Language::English,
            "pt" | "pt-br" | "pt-pt" | "portuguese" => Language::Portuguese,
            "es" | "spanish" | "espanol" => Language::Spanish,
            "de" | "german" | "deutsch" => Language::German,
            "fr" | "french" | "francais" => Language::French,
            "ru" | "russian" => Language::Russian,
            "ja" | "japanese" => Language::Japanese,
            _ => Language::Auto,
        }
    }

    pub fn display_info(&self) -> (&'static str, &'static str) {
        match self {
            Language::Auto => ("Auto (System)", "Detect OS Language"),
            Language::English => ("English (US)", "Default Global"),
            Language::Portuguese => ("Português (Brasil)", "Portuguese"),
            Language::Spanish => ("Español", "Spanish"),
            Language::German => ("Deutsch", "German"),
            Language::French => ("Français", "French"),
            Language::Russian => ("Русский", "Russian"),
            Language::Japanese => ("日本語", "Japanese"),
        }
    }

    pub fn all_available() -> &'static [Language] {
        &[
            Language::Auto,
            Language::English,
            Language::Portuguese,
            Language::Spanish,
            Language::German,
            Language::French,
            Language::Russian,
            Language::Japanese,
        ]
    }
}

const LEGACY_LANGUAGE_FILE: &str = ".litecord_language.json";

/// Absolute path for the persisted UI language choice (platform-standard
/// config dir; see `app_paths`). Replaces the old cwd-relative file, which is
/// still read as a fallback so existing users keep their choice.
pub fn language_config_path() -> std::path::PathBuf {
    super::app_paths::config_dir().join("language.json")
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LanguageConfig {
    pub selected_language: String,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            selected_language: "auto".to_string(),
        }
    }
}

/// Reads one language file. Returns `None` when absent or unparsable.
fn load_language_from_file(path: &std::path::Path) -> Option<Language> {
    let data = std::fs::read_to_string(path).ok()?;
    let cfg = serde_json::from_str::<LanguageConfig>(&data).ok()?;
    match Language::from_code(&cfg.selected_language) {
        Language::Auto => None,
        specific => Some(specific),
    }
}

/// Bias for the legacy cwd-relative file: reads `<cwd>/.litecord_language.json`.
/// The legacy file is never written or deleted (design: carry-over only).
pub fn load_persisted_language_config_at(cwd: &std::path::Path) -> Language {
    load_language_from_file(&language_config_path())
        .or_else(|| load_language_from_file(&cwd.join(LEGACY_LANGUAGE_FILE)))
        .unwrap_or(Language::Auto)
}

pub fn load_persisted_language_config() -> Language {
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    load_persisted_language_config_at(&cwd)
}

pub fn save_persisted_language_config(lang: Language) {
    let cfg = LanguageConfig {
        selected_language: lang.code().to_string(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&cfg) {
        let path = language_config_path();
        let _ = std::fs::create_dir_all(path.parent().unwrap_or(std::path::Path::new(".")));
        let _ = std::fs::write(path, json);
    }
}

#[cfg(target_os = "windows")]
pub fn detect_os_language() -> Language {
    if let Ok(lang) = std::env::var("LANG").or_else(|_| std::env::var("LC_ALL")) {
        let lower = lang.to_lowercase();
        if lower.starts_with("pt") { return Language::Portuguese; }
        if lower.starts_with("es") { return Language::Spanish; }
        if lower.starts_with("de") { return Language::German; }
        if lower.starts_with("fr") { return Language::French; }
        if lower.starts_with("ru") { return Language::Russian; }
        if lower.starts_with("ja") { return Language::Japanese; }
    }
    unsafe {
        let lang_id = windows_sys::Win32::Globalization::GetUserDefaultUILanguage();
        let primary_lang = lang_id & 0x03FF;
        match primary_lang {
            0x0016 => Language::Portuguese, // LANG_PORTUGUESE
            0x000A => Language::Spanish,    // LANG_SPANISH
            0x0007 => Language::German,     // LANG_GERMAN
            0x000C => Language::French,     // LANG_FRENCH
            0x0019 => Language::Russian,    // LANG_RUSSIAN
            0x0011 => Language::Japanese,   // LANG_JAPANESE
            _ => Language::English,
        }
    }
}

/// Reads the user's preferred languages from the macOS preferences
/// (`defaults read -g AppleLanguages`). Private to the macOS arm; errors and
/// "not found" collapse to `None` so the env fallback takes over.
#[cfg(target_os = "macos")]
pub fn detect_apple_language() -> Option<Language> {
    let out = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleLanguages"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    parse_apple_languages(&String::from_utf8_lossy(&out.stdout))
}

/// First language from Apple's `AppleLanguages` serialization that we support.
/// Handles single values ("en-US", "ja-JP"), quoted values, and plist arrays
/// spanning lines: unquoted `defaults` output uses plain `("en-US", "pt-BR")`.
#[cfg(target_os = "macos")]
pub fn parse_apple_languages(raw: &str) -> Option<Language> {
    let cleaned = raw
        .split(|c: char| c == '(' || c == ')' || c == ',' || c == '"' || c.is_whitespace());
    for token in cleaned {
        let code = token.to_lowercase();
        if code.is_empty() {
            continue;
        }
        match Language::from_code(&code) {
            Language::Auto => {
                // Region tags ("en-US") reduce to the base language first.
                let base = code.split('-').next().unwrap_or(&code);
                match Language::from_code(base) {
                    Language::Auto => continue,
                    mapped => return Some(mapped),
                }
            }
            mapped => return Some(mapped),
        }
    }
    None
}

/// macOS: `AppleLanguages` preferences first (a Finder-launched `.app` has no
/// `LANG`), falling back to the env vars, then English.
#[cfg(target_os = "macos")]
pub fn detect_os_language() -> Language {
    if let Some(lang) = detect_apple_language() {
        return lang;
    }
    detect_os_language_env()
}

#[cfg(target_os = "linux")]
pub fn detect_os_language() -> Language {
    detect_os_language_env()
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos"), not(target_os = "linux")))]
pub fn detect_os_language() -> Language {
    detect_os_language_env()
}

fn detect_os_language_env() -> Language {
    if let Ok(lang) = std::env::var("LANG").or_else(|_| std::env::var("LC_ALL")).or_else(|_| std::env::var("LC_MESSAGES")) {
        let lower = lang.to_lowercase();
        if lower.starts_with("pt") { return Language::Portuguese; }
        if lower.starts_with("es") { return Language::Spanish; }
        if lower.starts_with("de") { return Language::German; }
        if lower.starts_with("fr") { return Language::French; }
        if lower.starts_with("ru") { return Language::Russian; }
        if lower.starts_with("ja") { return Language::Japanese; }
    }
    Language::English
}

#[cfg(test)]
mod paths_and_detection {
    use super::*;
    use std::path::PathBuf;

    /// `defaults read -g AppleLanguages` outputs one of these shapes:
    /// a single line "en-US", a plist array across lines, or quoted values.
    #[cfg(target_os = "macos")]
    #[test]
    fn parse_apple_languages_variants() {
        assert_eq!(parse_apple_languages("en-US"), Some(Language::English));
        assert_eq!(
            parse_apple_languages("(\n  \"pt-BR\",\n  \"en-US\"\n)"),
            Some(Language::Portuguese)
        );
        assert_eq!(parse_apple_languages("\"ja-JP\""), Some(Language::Japanese));
        assert_eq!(parse_apple_languages("\"xx\""), None);
    }

    /// HOME choreography must not race with the rest of the suite, so the
    /// platform-path and persistence-migration assertions live in one test.
    #[test]
    fn config_and_persistence_paths() {
        let tmp = std::env::temp_dir().join("litecord-i18n-paths-test");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        let old_home = std::env::var_os("HOME");
        std::env::set_var("HOME", &tmp);
        std::env::remove_var("XDG_CONFIG_HOME");

        // Platform-standard location (design: OS convention, not vault-style).
        #[cfg(target_os = "macos")]
        {
            assert_eq!(
                super::super::app_paths::config_dir(),
                tmp.join("Library/Application Support/Litecord")
            );
        }
        #[cfg(target_os = "linux")]
        {
            assert_eq!(super::super::app_paths::config_dir(), tmp.join(".config").join("litecord"));
        }

        // 1) Only the legacy cwd-relative file exists → its choice survives.
        let legacy_cfg = LanguageConfig { selected_language: "pt".to_string() };
        let legacy_path = tmp.join(".litecord_language.json");
        std::fs::write(
            &legacy_path,
            serde_json::to_string(&legacy_cfg).unwrap(),
        )
        .unwrap();
        assert_eq!(
            load_persisted_language_config_at(&tmp),
            Language::Portuguese,
            "legacy fallback must carry existing users over"
        );

        // 2) Both files exist → the new location wins.
        let proper = language_config_path();
        std::fs::create_dir_all(proper.parent().unwrap()).unwrap();
        let ja_cfg = LanguageConfig { selected_language: "ja".to_string() };
        std::fs::write(&proper, serde_json::to_string(&ja_cfg).unwrap()).unwrap();
        assert_eq!(
            load_persisted_language_config_at(&tmp),
            Language::Japanese,
            "the new platform path must win over the legacy one"
        );

        match old_home {
            Some(h) => std::env::set_var("HOME", h),
            None => std::env::remove_var("HOME"),
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
