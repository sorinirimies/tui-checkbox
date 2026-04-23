#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-checkbox — Bump crate version
# ──────────────────────────────────────────────────────────────────────────────
# Usage:
#   nu scripts/bump_version.nu <new_version>
#
# Example:
#   nu scripts/bump_version.nu 0.5.0
#
# What it does:
#   1. Validates the supplied semantic version string.
#   2. Updates `package.version` in Cargo.toml.
#   3. Updates the version badge in README.md (if present).
#   4. Runs `cargo fmt`, `cargo clippy`, and `cargo test`.
#   5. Generates / updates the CHANGELOG via git-cliff (if installed).
#   6. Creates a Git commit and an annotated tag.
# ──────────────────────────────────────────────────────────────────────────────

# Validate that a string looks like a semver (MAJOR.MINOR.PATCH with optional pre-release).
def validate_version [version: string] {
    let pattern = '^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?$'
    if ($version | find --regex $pattern | is-empty) {
        print $"(ansi red)Error:(ansi reset) '($version)' is not a valid semantic version."
        exit 1
    }
}

# Replace the package.version line in Cargo.toml.
def update_package_version [version: string] {
    let cargo = (open Cargo.toml --raw)
    let updated = ($cargo | str replace --regex '(?m)^version\s*=\s*"[^"]+"' $'version = "($version)"')
    $updated | save --force Cargo.toml
    print $"(ansi green)✓(ansi reset) Updated package.version → ($version)"
}

# Update the crates.io version badge in README.md (if the badge exists).
def update_readme_badge [version: string] {
    if not ("README.md" | path exists) {
        print $"(ansi yellow)⚠(ansi reset) README.md not found — skipping badge update."
        return
    }
    let readme = (open README.md --raw)
    if ($readme =~ 'version-[0-9]+\.[0-9]+\.[0-9]+-blue') {
        let updated = (
            $readme
            | str replace --all --regex 'version-[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?-blue' $"version-($version)-blue"
        )
        $updated | save --force README.md
        print $"(ansi green)✓(ansi reset) Updated README.md version badge."
    } else {
        print $"(ansi yellow)⚠(ansi reset) No version badge found in README.md — skipping."
    }
}

def main [
    new_version: string,  # New version in X.Y.Z format
    --yes (-y),           # Skip confirmation prompt (non-interactive)
] {
    print ""
    print $"(ansi cyan)══════════════════════════════════════════════════════════════(ansi reset)"
    print $"(ansi cyan)  tui-checkbox — Bump Version(ansi reset)"
    print $"(ansi cyan)══════════════════════════════════════════════════════════════(ansi reset)"
    print ""

    # 0. Read current version
    let current_version = (open Cargo.toml | get package.version)
    print $"  Current version : (ansi yellow)($current_version)(ansi reset)"
    print $"  New version     : (ansi green)($new_version)(ansi reset)"
    print ""

    if $current_version == $new_version {
        print $"(ansi yellow)⚠(ansi reset) Version is already ($new_version). Nothing to do."
        exit 0
    }

    # 1. Validate
    validate_version $new_version
    print $"(ansi green)✓(ansi reset) Version string validated."

    # 2. Update package version
    update_package_version $new_version

    # 3. Update README badge
    update_readme_badge $new_version

    # 4. Update Cargo.lock
    print ""
    print $"(ansi cyan)── cargo update ────────────────────────────────────────────(ansi reset)"
    cargo update -p tui-checkbox
    print $"(ansi green)✓(ansi reset) Cargo.lock updated."

    # 5. cargo fmt
    print ""
    print $"(ansi cyan)── cargo fmt ───────────────────────────────────────────────(ansi reset)"
    cargo fmt
    print $"(ansi green)✓(ansi reset) cargo fmt completed."

    # 6. cargo clippy
    print ""
    print $"(ansi cyan)── cargo clippy ────────────────────────────────────────────(ansi reset)"
    cargo clippy -- -D warnings
    print $"(ansi green)✓(ansi reset) cargo clippy passed."

    # 7. cargo test
    print ""
    print $"(ansi cyan)── cargo test ──────────────────────────────────────────────(ansi reset)"
    cargo test --all-features --all-targets
    print $"(ansi green)✓(ansi reset) cargo test passed."

    # 8. Changelog (git-cliff)
    print ""
    print $"(ansi cyan)── changelog ───────────────────────────────────────────────(ansi reset)"
    if (which git-cliff | is-not-empty) {
        git-cliff --output CHANGELOG.md --tag $"v($new_version)"
        print $"(ansi green)✓(ansi reset) CHANGELOG.md updated via git-cliff."
    } else {
        print $"(ansi yellow)⚠(ansi reset) git-cliff not found — skipping changelog generation."
    }

    # 9. Git commit & tag
    print ""
    print $"(ansi cyan)── git commit & tag ────────────────────────────────────────(ansi reset)"
    git add -A
    git commit -m $"chore: bump version to ($new_version)"
    git tag -a $"v($new_version)" -m $"Release v($new_version)"
    print $"(ansi green)✓(ansi reset) Committed and tagged v($new_version)."

    print ""
    print $"(ansi green)══════════════════════════════════════════════════════════════(ansi reset)"
    print $"(ansi green)  tui-checkbox version bumped to ($new_version) 🚀(ansi reset)"
    print $"(ansi green)══════════════════════════════════════════════════════════════(ansi reset)"
    print ""
    print "  Next steps:"
    print $"    git push origin main --tags"
    print ""
}
