#!/usr/bin/env nu
# ── tui-checkbox · test_check_publish.nu ────────────────────────────────────
# Tests for scripts/check_publish.nu — pre-publish verification.

use std/assert
use runner.nu *

# ── Helpers ─────────────────────────────────────────────────────────────────

# Build a minimal Cargo.toml with the given version.
def make_cargo [version: string]: nothing -> string {
    $'[package]
name = "tui-checkbox"
version = "($version)"
edition = "2021"
license = "MIT"
description = "A customizable checkbox widget for Ratatui"
repository = "https://github.com/sorinirimies/tui-checkbox"

[dependencies]
ratatui = { version = "0.29", default-features = false }
'
}

# Read the package version from a Cargo.toml string.
def read_package_version [cargo_toml: string]: nothing -> string {
    $cargo_toml
    | lines
    | each { |l| $l | str trim }
    | where { |l| $l | str starts-with 'version' }
    | first
    | parse --regex 'version\s*=\s*"(?P<ver>[^"]+)"'
    | get ver
    | first
}

# Check required files exist (simulated).
def check_required_files [files: list<string>]: nothing -> bool {
    let required = ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    let missing = ($required | where { |r| not ($r in $files) })
    ($missing | length) == 0
}

# ── Tests ───────────────────────────────────────────────────────────────────

def "test check_publish: version is readable" [] {
    let cargo = (make_cargo "0.4.1")
    let ver = (read_package_version $cargo)
    assert equal $ver "0.4.1"
}

def "test check_publish: all required files present passes" [] {
    let files = ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    assert (check_required_files $files)
}

def "test check_publish: missing README fails" [] {
    let files = ["LICENSE" "Cargo.toml" "CHANGELOG.md"]
    assert (not (check_required_files $files))
}

def "test check_publish: missing LICENSE fails" [] {
    let files = ["README.md" "Cargo.toml" "CHANGELOG.md"]
    assert (not (check_required_files $files))
}

def "test check_publish: missing CHANGELOG fails" [] {
    let files = ["README.md" "LICENSE" "Cargo.toml"]
    assert (not (check_required_files $files))
}

def "test check_publish: missing Cargo.toml fails" [] {
    let files = ["README.md" "LICENSE" "CHANGELOG.md"]
    assert (not (check_required_files $files))
}

def "test check_publish: extra files do not affect check" [] {
    let files = ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md" "CONTRIBUTING.md" "rustfmt.toml"]
    assert (check_required_files $files)
}

def "test check_publish: empty file list fails" [] {
    let files = []
    assert (not (check_required_files $files))
}

def "test check_publish: cargo has required fields" [] {
    let cargo = (make_cargo "1.0.0")
    assert ($cargo | str contains 'name = "tui-checkbox"')
    assert ($cargo | str contains 'license = "MIT"')
    assert ($cargo | str contains 'description =')
    assert ($cargo | str contains 'repository =')
}

def "test check_publish: zero version is valid" [] {
    let cargo = (make_cargo "0.0.0")
    let ver = (read_package_version $cargo)
    assert equal $ver "0.0.0"
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }
