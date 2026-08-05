#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-checkbox — Compute next patch version (downgrade-safe)
# ──────────────────────────────────────────────────────────────────────────────
# Usage:
#   nu scripts/next_patch_version.nu <current_version>            # simple bump
#   nu scripts/next_patch_version.nu <current_version> --safe     # guarded bump
#
# The `--safe` mode is used by the nightly dependency-upgrade workflow. It
# guards against ever producing a version that would:
#   • be lower than or equal to the highest existing `vX.Y.Z` git tag, and
#   • collide with a version already published on crates.io.
#
# This matters because the nightly job always bumps from whatever is in
# Cargo.toml on `main`. If a manual release/tag ever gets ahead of that (or a
# previous nightly run partially failed after tagging but before crates.io
# publish), a naive "+1 patch" could produce a tag/version that is behind —
# effectively a downgrade — or a duplicate that fails to publish.
# ──────────────────────────────────────────────────────────────────────────────

use ci/publish.nu [is-published]

# Parse a MAJOR.MINOR.PATCH (optionally with a `-prerelease` suffix, which is
# dropped) semver string into its numeric components.
export def parse-semver [version: string]: nothing -> record<major: int, minor: int, patch: int> {
    let pattern = '^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(-.*)?$'
    let parsed = ($version | parse --regex $pattern)

    if ($parsed | is-empty) {
        error make { msg: $"'($version)' is not a valid semantic version" }
    }

    let row = ($parsed | first)
    {
        major: ($row.major | into int),
        minor: ($row.minor | into int),
        patch: ($row.patch | into int),
    }
}

# Given a MAJOR.MINOR.PATCH version string, return the next patch version
# (pre-release suffixes are dropped).
export def next-patch [version: string]: nothing -> string {
    let v = (parse-semver $version)
    $"($v.major).($v.minor).($v.patch + 1)"
}

# Compare two semver strings. Returns -1, 0, or 1 (like a normal comparator).
export def compare-semver [a: string, b: string]: nothing -> int {
    let va = (parse-semver $a)
    let vb = (parse-semver $b)

    if $va.major != $vb.major {
        if $va.major < $vb.major { -1 } else { 1 }
    } else if $va.minor != $vb.minor {
        if $va.minor < $vb.minor { -1 } else { 1 }
    } else if $va.patch != $vb.patch {
        if $va.patch < $vb.patch { -1 } else { 1 }
    } else {
        0
    }
}

# Return the highest `vX.Y.Z` version among the given git tags (as bare
# "X.Y.Z" strings, without the "v" prefix). Tags that don't match the
# expected shape are ignored. Returns null if there are no matching tags.
export def highest-tag-version [tags: list<string>]: nothing -> any {
    let versions = (
        $tags
        | where { |t| $t | str starts-with "v" }
        | each { |t| $t | str substring 1.. }
        | where { |v| (try { parse-semver $v; true } catch { false }) }
    )

    if ($versions | is-empty) {
        null
    } else {
        $versions | reduce { |it, acc| if (compare-semver $it $acc) > 0 { $it } else { $acc } }
    }
}

# Compute a downgrade-safe next version:
#   1. Start from the higher of `current_version` and the highest existing
#      `vX.Y.Z` git tag (so a stale Cargo.toml on `main` never regresses a
#      version that was already tagged/released elsewhere).
#   2. Bump the patch component.
#   3. If that version is already published on crates.io, keep bumping the
#      patch until an unpublished version is found.
export def safe-next-version [
    current_version: string,
    crate_name: string,
    tags: list<string>,
]: nothing -> string {
    let highest_tag = (highest-tag-version $tags)

    let base = if $highest_tag == null {
        $current_version
    } else if (compare-semver $highest_tag $current_version) > 0 {
        $highest_tag
    } else {
        $current_version
    }

    mut candidate = (next-patch $base)
    while (is-published $crate_name $candidate) {
        $candidate = (next-patch $candidate)
    }
    $candidate
}

def main [
    current_version: string,  # Current version from Cargo.toml
    --safe,                   # Guard against downgrades / crates.io collisions
    --crate-name: string = "", # Crate name (defaults to Cargo.toml package.name)
] {
    if $safe {
        let name = if $crate_name == "" {
            (open Cargo.toml | get package.name)
        } else {
            $crate_name
        }
        let tags = (try { git tag --list "v*" | lines } catch { [] })
        print (safe-next-version $current_version $name $tags)
    } else {
        print (next-patch $current_version)
    }
}
