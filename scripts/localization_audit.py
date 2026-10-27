#!/usr/bin/env python3
"""Localization audit inventory for Litecord.

Scans the Slint UI and Rust sources and emits docs/LOCALIZATION_AUDIT.md —
a check-list inventory of every user-visible string and which translation
pipeline currently serves it. Stdlib only, deterministic, no network.

Sections (rule classes, as per the localization-consistency plan):
  1. tr_* hand-wired   — Slint `tr_*` string properties fed by the old
      i18n::Translations struct, one set_tr_* call at a time (to migrate).
  2. i18n_ui           — strings already served by the Tr global.
  3. hardcoded slint   — UI text literals carrying Portuguese-only accented
      chars, never routed through i18n.
  4. log (out of scope)- Rust log literals in Portuguese, recorded only.
  5. hardcoded tray    — tray.rs menu/tooltip literals.
  6. Whitelist         — sanctioned bilingual/native-language strings, seeded
      from the design's exception list (consumed by the no-leak lint test).

The generated .md is committed; re-run and `git diff docs/LOCALIZATION_AUDIT.md`
to review inventory drift.
"""

from __future__ import annotations

import re
import subprocess
from collections import defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OUT = REPO / "docs" / "LOCALIZATION_AUDIT.md"

# Portuguese-only accented characters (not used by the other shipped languages).
PT_ONLY_CHARS = "íâãéêóôõúç"

SLINT_TEXT = re.compile(r'^\s*text:\s*"((?:[^"\\]|\\.)*)"')
SLINT_PROP_DEFAULT = re.compile(r'^\s*in(?:-out)?\s+property\s+<string>\s+(\w+)\s*:\s*"((?:[^"\\]|\\.)*)"')
SLINT_TR_REF = re.compile(r"Tr\.\w+")
SLINT_TR_PROP = re.compile(r"^\s*in(?:-out)?\s+property\s+<string>\s+(tr_\w+)\b")
RUST_LOG = re.compile(r"(info!|warn!|error!|debug!|println!)\s*\(")
TRAY_MENU = re.compile(r'(MenuItem::new\("([^"]+)"|with_tooltip\("([^"]+)")')

WHITELIST_SEED = """\
| file:line | reason |
|---|---|
| (i18n tables: native language names render in their own language by design) | native names |
| ui/components/*settings_view.slint:LANGUAGE / IDIOMA rows | permanent bilingual picker label |
"""


def git_glob(*patterns: str) -> list[Path]:
    """Stable, repo-relative file list from git (respects .gitignore).

    Note: git pathspec `dir/**/*.ext` matches only *nested* layers, so callers
    pass both `dir/*.ext` and `dir/**/*.ext`.
    """
    files: set[Path] = set()
    for pattern in patterns:
        out = subprocess.run(
            ["git", "ls-files", pattern], capture_output=True, text=True, check=True
        )
        files.update(REPO / p for p in out.stdout.splitlines() if p)
    return sorted(files)


def classify() -> dict[str, list[tuple[str, int, str]]]:
    rows: dict[str, list[tuple[str, int, str]]] = defaultdict(list)

    for path in git_glob("ui/*.slint", "ui/**/*.slint"):
        rel = path.relative_to(REPO)
        for lineno, line in enumerate(path.read_text(errors="replace").splitlines(), 1):
            prop = SLINT_TR_PROP.match(line)
            if prop:
                # tr_stream_quality is a dynamic runtime value (resolution+fps
                # set from src/main.rs), not a translated string — same exempt
                # as the guard test in src/utils/i18n_ui.rs.
                if prop.group(1) != "tr_stream_quality":
                    rows["1. tr_* hand-wired (Slint properties fed by old Translations)"].append(
                        (str(rel), lineno, prop.group(1))
                    )
                continue
            is_text_line = "text:" in line and "//" not in line.split("text:")[0]
            m = SLINT_TEXT.search(line) if is_text_line else None
            pd = SLINT_PROP_DEFAULT.match(line)
            if pd and any(c in pd.group(2) for c in PT_ONLY_CHARS):
                rows["3. hardcoded slint property defaults (Portuguese-typical chars)"].append(
                    (str(rel), lineno, f'{pd.group(1)}: "{pd.group(2)[:70]}"')
                )
                continue
            pd_default = pd and not SLINT_TR_REF.search(line)
            literal = (m.group(1) if m else (pd.group(2) if pd_default else None))
            if literal:
                if any(c in literal for c in PT_ONLY_CHARS):
                    rows["3. hardcoded slint literals (Portuguese-typical chars)"].append(
                        (str(rel), lineno, literal[:80])
                    )
                elif (
                    len(re.findall(r"[A-Za-zÀ-ÿ]{3,}", literal)) >= 2
                    and not literal.startswith(("https://", "ws"))
                ):
                    rows[
                        "3b. hardcoded slint literals (brand/untranslated ASCII, review)"
                    ].append((str(rel), lineno, literal[:80]))

    for path in git_glob("src/*.rs", "src/**/*.rs"):
        rel = path.relative_to(REPO)
        for lineno, line in enumerate(path.read_text(errors="replace").splitlines(), 1):
            if RUST_LOG.search(line) and any(c in line for c in PT_ONLY_CHARS):
                rows["4. Rust log literals in Portuguese (out of scope, recorded)"].append(
                    (str(rel), lineno, re.sub(r"\s+", " ", line.strip())[:90])
                )
            if rel.parts[-1] == "tray.rs":
                # Small surface: list every menu/tooltip literal unconditionally
                # (language spotting is human review; catching unaccented words
                # like "Ultra-Leve" by rule is not worth the complexity).
                for m in TRAY_MENU.finditer(line):
                    label = m.group(2) or m.group(3)
                    if label and not label.startswith(("https://", "ws")):
                        rows["5. hardcoded tray literals"].append(
                            (str(rel), lineno, label)
                        )

    return rows


def main() -> None:
    rows = classify()
    lines: list[str] = [
        "# Localization Audit",
        "",
        "Generated by `scripts/localization_audit.py` — do not edit by hand;",
        "edit the script and re-run. Checked rows belong to a tracking task in",
        "`docs/superpowers/plans/2026-10-26-localization-consistency.md`.",
        "",
    ]
    for section in sorted(rows, key=lambda s: s.split(".")[0].zfill(2)):
        lines += [f"## {section}", "", "| status | file:line | excerpt |", "|---|---|---|"]
        for relf, lineno, excerpt in rows[section]:
            lines.append(f"| [ ] | {relf}:{lineno} | {excerpt} |")
        lines.append(f"");
        lines.append(f"**{len(rows[section])} entries.**")
        lines.append("")

    if "6. Whitelist" not in rows:
        lines += ["## 6. Whitelist", "", WHITELIST_SEED, ""]

    OUT.write_text("\n".join(lines) + "\n")
    for section, entries in sorted(rows.items(), key=lambda kv: kv[0]):
        print(f"{section}: {len(entries)}")


if __name__ == "__main__":
    main()
