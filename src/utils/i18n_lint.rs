//! Test-support guardrails for the translation pipeline (the plan calls this
//! "the no-leak lint"). Runs only under `cargo test`.
//!
//! - `no_leaked_portuguese_literals`: any static `.slint` literal (`text: "…"`
//!   or `property <string> x: "…"`) carrying a Portuguese-only char fails,
//!   except lines listed under `## Whitelist` in docs/LOCALIZATION_AUDIT.md
//!   with rows of the form `| file:line |`.
//!
//! English is the universal fallback — see the design in
//! docs/superpowers/specs/2026-10-26-localization-consistency-design.md.

#[cfg(test)]
mod no_leak_lint {
    use std::path::PathBuf;

    const PT_ONLY: [char; 10] = ['í', 'â', 'ã', 'é', 'ê', 'ó', 'ô', 'õ', 'ú', 'ç'];
    const AUDIT: &str = "docs/LOCALIZATION_AUDIT.md";

    fn ui_slint_files() -> Vec<PathBuf> {
        let mut files = Vec::new();
        let mut stack = vec![concat!(env!("CARGO_MANIFEST_DIR"), "/ui").to_string()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    stack.push(p.to_string_lossy().into_owned());
                } else if p.extension().map(|e| e == "slint").unwrap_or(false) {
                    files.push(p);
                }
            }
        }
        files
    }

    /// `file:line` pairs allowed to keep Portuguese-only chars, parsed from
    /// the audit's `## Whitelist` section. Notes, headers, and non-directive
    /// rows are skipped so the audit can carry explanatory prose.
    fn whitelist() -> Vec<String> {
        let manifest = env!("CARGO_MANIFEST_DIR");
        let text = std::fs::read_to_string(format!("{manifest}/{AUDIT}")).unwrap_or_default();
        let mut out = Vec::new();
        let mut in_section = false;
        for line in text.lines() {
            if line.starts_with("## ") {
                in_section = line.starts_with("## Whitelist");
                continue;
            }
            if !in_section {
                continue;
            }
            let cells: Vec<&str> = line.trim().split('|').collect();
            if cells.len() < 3 {
                continue;
            }
            let anchor = cells[1].trim();
            if anchor.contains(':') && anchor.split(':').next().unwrap_or("").contains(".slint") {
                out.push(anchor.to_string());
            }
        }
        out
    }

    /// String literal contents on this line, in order. Handles backslash
    /// escapes; Slint literals are double-quoted like Rust's.
    fn literals(line: &str) -> Vec<String> {
        let mut out = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '"' {
                let mut buf = String::new();
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        buf.push(chars[i]);
                        i += 1;
                    }
                    buf.push(chars[i]);
                    i += 1;
                }
                i += 1;
                out.push(buf);
            } else {
                i += 1;
            }
        }
        out
    }

    #[test]
    fn no_leaked_portuguese_literals() {
        let allow = whitelist();
        let mut leaks: Vec<String> = Vec::new();
        for file in ui_slint_files() {
            let path_str = match file.strip_prefix(env!("CARGO_MANIFEST_DIR")) {
                Ok(p) => p.to_string_lossy().into_owned(),
                Err(_) => file.to_string_lossy().into_owned(),
            };
            let text = match std::fs::read_to_string(&file) {
                Ok(t) => t,
                Err(_) => continue,
            };
            for (idx, line) in text.lines().enumerate() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") || trimmed.is_empty() {
                    continue;
                }
                let is_text = trimmed.starts_with("text:");
                let is_prop_default = trimmed.starts_with("in property <string>")
                    || trimmed.starts_with("in-out property <string>");
                if !(is_text || is_prop_default) {
                    continue;
                }
                let anchor = format!("{path_str}:{}", idx + 1);
                for literal in literals(line) {
                    if PT_ONLY.iter().any(|c| literal.contains(*c)) && !allow.contains(&anchor) {
                        leaks.push(format!("{anchor}: {literal:?}"));
                    }
                }
            }
        }
        assert!(
            leaks.is_empty(),
            "Portuguese-only literals in the UI (whitelist only intentionally \
             bilingual strings like native language names): {leaks:?}"
        );
    }
}
