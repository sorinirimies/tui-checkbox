# tui-checkbox — task runner
# Install just:      cargo install just
# Install git-cliff: cargo install git-cliff
# Install nu:        cargo install nu
# Usage: just <task>

# ── Default ───────────────────────────────────────────────────────────────────────

default:
    @just --list

# ── Tool checks ───────────────────────────────────────────────────────────────────

_check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { \
        echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; \
    }

_check-nu:
    @command -v nu >/dev/null 2>&1 || { \
        echo "❌ nu (nushell) not found. Install: https://www.nushell.sh"; exit 1; \
    }

_check-vhs:
    @command -v vhs >/dev/null 2>&1 || { \
        echo "❌ vhs not found."; \
        echo "   macOS:      brew install vhs"; \
        echo "   Any:        go install github.com/charmbracelet/vhs@latest"; \
        exit 1; \
    }

# Install all recommended development tools
install-tools:
    @echo "Installing development tools…"
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff --locked
    @command -v nu >/dev/null 2>&1 || cargo install nu --locked
    @echo "✅ All tools installed!"

# ── Build ─────────────────────────────────────────────────────────────────────────────

# Build the project (dev)
build:
    cargo build

# Build release binary
build-release:
    cargo build --release

# ── Run ───────────────────────────────────────────────────────────────────────────────

# Run the checkbox example
run:
    cargo run --example checkbox

# Watch and auto-run on file changes (requires cargo-watch)
watch:
    cargo watch -x "run --example checkbox"

# ── Test ──────────────────────────────────────────────────────────────────────────────

# Run the test suite
test:
    cargo test --locked --all-features --all-targets

# Run Nu script tests
test-nu: _check-nu
    nu scripts/tests/run_all.nu

# Run both Rust and Nu tests
test-all: test test-nu
    @echo "✅ All Rust and Nu tests passed!"

# ── Code quality ──────────────────────────────────────────────────────────────────────

# Check without building
check:
    cargo check

# Format all code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt -- --check

# Run clippy
clippy:
    cargo clippy --all-targets --all-features -- -D warnings -A deprecated

# Run all quality checks (format, clippy, test, nu) — must pass before a release.
check-all: fmt clippy test test-nu
    @echo "🔍 Verifying formatting is clean…"
    cargo fmt -- --check
    @echo "✅ All checks passed!"

# Full pre-release quality gate — everything in check-all plus a release build.
check-release: check-all build-release
    @echo "✅ Release quality gate passed (fmt + clippy + test + nu + release build)!"

# ── VHS Demo GIFs ─────────────────────────────────────────────────────────────────────

# Run the VHS tape to generate demo GIF
vhs: _check-vhs
    @echo "Running VHS tape to generate demo…"
    vhs examples/checkbox.tape
    @echo "✅ Demo generated!"

# ── Documentation ─────────────────────────────────────────────────────────────────────

# Generate and open docs
doc:
    cargo doc --no-deps --open

# ── Changelog ─────────────────────────────────────────────────────────────────────────

# Regenerate the full CHANGELOG.md from all tags
changelog: _check-git-cliff
    @echo "Generating full changelog…"
    git-cliff --output CHANGELOG.md
    @echo "✅ CHANGELOG.md updated."

# Prepend only unreleased commits to CHANGELOG.md
changelog-unreleased: _check-git-cliff
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Unreleased changes prepended."

# Preview changelog for the next release without writing the file
changelog-preview: _check-git-cliff
    @git-cliff --unreleased

# ── Version bump ─────────────────────────────────────────────────────────────────────

# Validate that a version string will produce a valid vX.Y.Z tag.
validate-tag version: _check-nu
    @nu scripts/ci/validate_tag.nu "v{{ version }}" 2>&1 >/dev/null

# Fail fast if the requested version is the same as the current one.
_check-version-changed version: _check-nu
    #!/usr/bin/env sh
    current=$(nu scripts/version.nu)
    if [ "$current" = "{{ version }}" ]; then
        echo "❌ Version {{ version }} is already the current version. Nothing to bump."
        exit 1
    fi
    echo "✅ Version will change: $current → {{ version }}"

# Bump the version, regenerate Cargo.lock + CHANGELOG.md, commit and tag.
bump version: (validate-tag version) (_check-version-changed version) check-release _check-git-cliff
    nu scripts/bump_version.nu --yes {{ version }}

# ── Publish (crates.io) ─────────────────────────────────────────────────────────────

# Run the full pre-publish readiness check (fmt, clippy, tests, docs, dry-run)
check-publish: _check-nu
    nu scripts/check_publish.nu

# Dry-run publish (verifies packaging; skips if version already on crates.io)
publish-dry: check-all _check-nu
    @echo "Dry-run: tui-checkbox"
    nu scripts/ci/publish.nu --dry-run

# Publish to crates.io (idempotent: skips if this version already exists)
publish: check-all _check-nu
    @echo "📦 Publishing tui-checkbox…"
    nu scripts/ci/publish.nu
    @echo "✅ tui-checkbox publish step complete!"

# Show what would be released without making any changes
release-preview: _check-git-cliff
    @echo "Current version: $(just version)"
    @echo ""
    @echo "Unreleased commits:"
    @git-cliff --unreleased
    @echo ""
    @echo "Package version:"
    @grep '^version' Cargo.toml | head -1
    @echo ""
    @echo "Published crate: tui-checkbox"

# ── Housekeeping ──────────────────────────────────────────────────────────────────────

# Remove build artifacts
clean:
    cargo clean

# Update all dependencies (Cargo.lock only)
update:
    cargo update

# Update dependencies, run the full quality gate, then commit and push if all green.
update-deps:
    @echo "⬆️  Updating dependencies…"
    cargo update
    @echo "🔍 Running quality gate…"
    cargo fmt -- --check
    cargo clippy --all-targets --all-features -- -D warnings -A deprecated
    cargo test --locked --all-features --all-targets
    @echo "✅ All checks passed — committing dependency updates…"
    git add Cargo.lock
    git diff --cached --quiet || git commit -m "chore: update dependencies"
    git push origin main
    @echo "✅ Dependency updates pushed to GitHub."

# Show outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# Show the current version
version: _check-nu
    @nu scripts/version.nu

# Show all configured remotes
remotes:
    @git remote -v

# Show project info
info:
    @echo "Project: tui-checkbox"
    @echo "Version: $(just version)"
    @echo "Author: Sorin Albu-Irimies"
    @echo "License: MIT"

# ── Git remotes & pushing ────────────────────────────────────────────────────────────

# Push the current branch to GitHub (origin)
push:
    git push origin main

# Push the current branch to Gitea Microlab
push-gitea-microlab:
    git push gitea-microlab main

# Push the current branch to Gitea Starscream
push-gitea-starscream:
    git push gitea-starscream main

# Push the current branch to Gitea (nexus-lab instance)
push-gitea-nexus-lab:
    git push gitea-nexus-lab main

# Push the current branch to all remotes (continues on failure)
push-all:
    #!/usr/bin/env sh
    failed=""
    git push origin main             || failed="$failed origin"
    git push gitea-microlab main              || failed="$failed gitea-microlab"
    git push gitea-starscream main   || failed="$failed gitea-starscream"
    git push gitea-nexus-lab main    || failed="$failed gitea-nexus-lab"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to push to:$failed"
    else
        echo "✅ Pushed to GitHub, Gitea, Gitea Starscream, and Gitea (nexus-lab)!"
    fi

# Force-push the current branch to all remotes
push-all-force:
    #!/usr/bin/env sh
    failed=""
    git push --force origin main             || failed="$failed origin"
    git push --force gitea-microlab main              || failed="$failed gitea-microlab"
    git push --force gitea-starscream main   || failed="$failed gitea-starscream"
    git push --force gitea-nexus-lab main    || failed="$failed gitea-nexus-lab"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to force-push to:$failed"
    else
        echo "✅ Force-pushed to GitHub, Gitea, Gitea Starscream, and Gitea (nexus-lab)!"
    fi

# Pull the current branch from GitHub (origin)
pull:
    git pull origin main

# Pull the current branch from Gitea Microlab
pull-gitea-microlab:
    git pull gitea-microlab main

# Pull the current branch from Gitea Starscream
pull-gitea-starscream:
    git pull gitea-starscream main

# Pull the current branch from Gitea (nexus-lab instance)
pull-gitea-nexus-lab:
    git pull gitea-nexus-lab main

# Pull the current branch from all remotes (continues on failure)
pull-all:
    #!/usr/bin/env sh
    failed=""
    git pull origin main             || failed="$failed origin"
    git pull gitea-microlab main              || failed="$failed gitea-microlab"
    git pull gitea-starscream main   || failed="$failed gitea-starscream"
    git pull gitea-nexus-lab main    || failed="$failed gitea-nexus-lab"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to pull from:$failed"
    else
        echo "✅ Pulled from GitHub, Gitea, Gitea Starscream, and Gitea (nexus-lab)!"
    fi

# Push all tags to GitHub
push-tags:
    git push origin --tags

# Push all tags to all remotes (continues on failure)
push-tags-all:
    #!/usr/bin/env sh
    failed=""
    git push origin --tags             || failed="$failed origin"
    git push gitea-microlab --tags              || failed="$failed gitea-microlab"
    git push gitea-starscream --tags   || failed="$failed gitea-starscream"
    git push gitea-nexus-lab --tags    || failed="$failed gitea-nexus-lab"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to push tags to:$failed"
    else
        echo "✅ Tags pushed to all remotes!"
    fi

# ── Release workflows ───────────────────────────────────────────────────────────────

# Bump, commit, tag, then push to GitHub — triggers Release workflow.
release version: (bump version)
    @echo "Pushing release v{{ version }} to GitHub…"
    git push --follow-tags origin main
    @echo "✅ Release v{{ version }} pushed — Release workflow will trigger automatically."

# Bump, commit, tag, then push to Gitea Microlab only.
release-gitea-microlab version: (bump version)
    @echo "Pushing release v{{ version }} to Gitea Microlab…"
    git push --follow-tags gitea-microlab main
    @echo "✅ Release v{{ version }} live on Gitea Microlab."

# Bump, commit, tag, then push to Gitea Starscream only.
release-gitea-starscream version: (bump version)
    @echo "Pushing release v{{ version }} to Gitea Starscream…"
    git push --follow-tags gitea-starscream main
    @echo "✅ Release v{{ version }} live on Gitea Starscream."

# Bump, commit, tag, then push to Gitea (nexus-lab instance) only.
release-gitea-nexus-lab version: (bump version)
    @echo "Pushing release v{{ version }} to Gitea (nexus-lab)…"
    git push --follow-tags gitea-nexus-lab main
    @echo "✅ Release v{{ version }} live on Gitea (nexus-lab)."

# Bump, commit, tag, then push to all remotes (continues on failure).
release-all version: (bump version)
    #!/usr/bin/env sh
    echo "Pushing release v{{ version }} to all remotes…"
    failed=""
    git push --follow-tags origin main             || failed="$failed origin"
    git push --follow-tags gitea-microlab main              || failed="$failed gitea-microlab"
    git push --follow-tags gitea-starscream main   || failed="$failed gitea-starscream"
    git push --follow-tags gitea-nexus-lab main    || failed="$failed gitea-nexus-lab"
    if [ -n "$failed" ]; then
        echo "⚠️  Release v{{ version }} failed to push to:$failed"
    else
        echo "✅ Release v{{ version }} pushed to GitHub, Gitea, Gitea Starscream, and Gitea (nexus-lab)!"
    fi

# Push the latest commit and all tags to every remote (no bump, continues on failure).
push-release-all: check-all
    #!/usr/bin/env sh
    failed=""
    git push --follow-tags origin main             || failed="$failed origin"
    git push --follow-tags gitea-microlab main              || failed="$failed gitea-microlab"
    git push --follow-tags gitea-starscream main   || failed="$failed gitea-starscream"
    git push --follow-tags gitea-nexus-lab main    || failed="$failed gitea-nexus-lab"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to push to:$failed"
    else
        echo "✅ Latest commit + tags pushed to all remotes."
    fi

# Force-sync Gitea Microlab with GitHub
sync-gitea-microlab:
    git push gitea-microlab main --force
    git push gitea-microlab --tags --force
    @echo "✅ Gitea Microlab force-synced with GitHub."

# Force-sync Gitea Starscream with GitHub
sync-gitea-starscream:
    git push gitea-starscream main --force
    git push gitea-starscream --tags --force
    @echo "✅ Gitea Starscream force-synced with GitHub."

# Force-sync Gitea (nexus-lab instance) with GitHub
sync-gitea-nexus-lab:
    git push gitea-nexus-lab main --force
    git push gitea-nexus-lab --tags --force
    @echo "✅ Gitea (nexus-lab) force-synced with GitHub."

# Force-sync all Gitea instances with GitHub (continues on failure)
sync-all-gitea:
    #!/usr/bin/env sh
    failed=""
    git push gitea-microlab main --force                  || failed="$failed gitea-microlab"
    git push gitea-microlab --tags --force                || failed="$failed gitea-microlab-tags"
    git push gitea-starscream main --force       || failed="$failed gitea-starscream"
    git push gitea-starscream --tags --force     || failed="$failed gitea-starscream-tags"
    if [ -n "$failed" ]; then
        echo "⚠️  Failed to sync:$failed"
    else
        echo "✅ All Gitea instances force-synced with GitHub."
    fi

# Add a Gitea remote and optionally push — interactive (nu script)
setup-gitea url: _check-nu
    nu scripts/setup_gitea.nu {{ url }}

# Migrate this project to dual GitHub + Gitea hosting (interactive)
migrate-gitea: _check-nu
    nu scripts/migrate_to_gitea.nu
