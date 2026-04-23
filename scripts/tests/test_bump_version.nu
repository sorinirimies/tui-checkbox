#!/usr/bin/env nu
# ── tui-checkbox · test_bump_version.nu ─────────────────────────────────────
# Tests for scripts/bump_version.nu — bumping versions in Cargo.toml.

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

# Update the package version in a Cargo.toml string.
def bump_version [cargo_toml: string, new_version: string]: nothing -> string {
    $cargo_toml | str replace --regex '(?m)^version\s*=\s*"[^"]+"' $'version = "($new_version)"'
}

# ── Tests ───────────────────────────────────────────────────────────────────

def "test bump_version: patch bump updates version" [] {
    let cargo = (make_cargo "1.0.0")
    let bumped = (bump_version $cargo "1.0.1")
    let ver = (read_package_version $bumped)
    assert equal $ver "1.0.1"
}

def "test bump_version: minor bump updates version" [] {
    let cargo = (make_cargo "1.0.0")
    let bumped = (bump_version $cargo "1.1.0")
    let ver = (read_package_version $bumped)
    assert equal $ver "1.1.0"
}

def "test bump_version: major bump updates version" [] {
    let cargo = (make_cargo "1.2.3")
    let bumped = (bump_version $cargo "2.0.0")
    let ver = (read_package_version $bumped)
    assert equal $ver "2.0.0"
}

def "test bump_version: pre-release version" [] {
    let cargo = (make_cargo "1.0.0")
    let bumped = (bump_version $cargo "1.1.0-rc.1")
    let ver = (read_package_version $bumped)
    assert equal $ver "1.1.0-rc.1"
}

def "test bump_version: idempotent when version unchanged" [] {
    let cargo = (make_cargo "0.5.0")
    let bumped = (bump_version $cargo "0.5.0")
    let ver = (read_package_version $bumped)
    assert equal $ver "0.5.0"
}

def "test bump_version: double bump produces correct version" [] {
    let cargo = (make_cargo "0.1.0")
    let first = (bump_version $cargo "0.2.0")
    let second = (bump_version $first "0.3.0")
    let ver = (read_package_version $second)
    assert equal $ver "0.3.0"
}

def "test bump_version: preserves package name" [] {
    let cargo = (make_cargo "1.0.0")
    let bumped = (bump_version $cargo "2.0.0")
    assert ($bumped | str contains 'name = "tui-checkbox"')
}

def "test bump_version: preserves dependencies" [] {
    let cargo = (make_cargo "1.0.0")
    let bumped = (bump_version $cargo "2.0.0")
    assert ($bumped | str contains '[dependencies]')
    assert ($bumped | str contains 'ratatui')
}

def "test bump_version: preserves edition" [] {
    let cargo = (make_cargo "1.0.0")
    let bumped = (bump_version $cargo "2.0.0")
    assert ($bumped | str contains 'edition = "2021"')
}

def "test bump_version: zero to non-zero" [] {
    let cargo = (make_cargo "0.0.0")
    let bumped = (bump_version $cargo "0.1.0")
    let ver = (read_package_version $bumped)
    assert equal $ver "0.1.0"
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }
