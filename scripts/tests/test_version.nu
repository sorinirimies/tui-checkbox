#!/usr/bin/env nu
# ── tui-checkbox · test_version.nu ──────────────────────────────────────────
# Tests for scripts/version.nu — reading the crate version from Cargo.toml.

use std/assert
use runner.nu *

# ── Helpers ─────────────────────────────────────────────────────────────────

# Build a minimal Cargo.toml string with the given version.
def make_cargo [version: string]: nothing -> string {
    $'[package]
name = "tui-checkbox"
version = "($version)"
edition = "2021"
license = "MIT"
description = "A customizable checkbox widget for Ratatui"

[dependencies]
ratatui = { version = "0.29", default-features = false }
'
}

# Extract the package version the same way version.nu does.
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

# ── Tests ───────────────────────────────────────────────────────────────────

def "test version: reads simple semver" [] {
    let cargo = (make_cargo "1.2.3")
    let ver = (read_package_version $cargo)
    assert equal $ver "1.2.3"
}

def "test version: reads pre-release version" [] {
    let cargo = (make_cargo "0.5.0-rc.1")
    let ver = (read_package_version $cargo)
    assert equal $ver "0.5.0-rc.1"
}

def "test version: reads zero version" [] {
    let cargo = (make_cargo "0.0.0")
    let ver = (read_package_version $cargo)
    assert equal $ver "0.0.0"
}

def "test version: reads large version numbers" [] {
    let cargo = (make_cargo "12.345.6789")
    let ver = (read_package_version $cargo)
    assert equal $ver "12.345.6789"
}

def "test version: rejects cargo without version" [] {
    let cargo = '[package]
name = "tui-checkbox"
edition = "2021"
'
    let failed = (try { read_package_version $cargo; false } catch { true })
    assert $failed
}

def "test version: version matches expected format" [] {
    let cargo = (make_cargo "0.4.1")
    let ver = (read_package_version $cargo)
    let is_semver = ($ver | find --regex '^\d+\.\d+\.\d+' | is-not-empty)
    assert $is_semver
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }
