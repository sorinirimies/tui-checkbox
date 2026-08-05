#!/usr/bin/env nu
# ── tui-checkbox · test_next_patch_version.nu ───────────────────────────────
# Tests for scripts/next_patch_version.nu — patch-version bump used by the
# nightly dependency-upgrade workflow to compute the next release tag.

use std/assert
use runner.nu *
use ../next_patch_version.nu [parse-semver, next-patch, compare-semver, highest-tag-version, safe-next-version]

# ── parse-semver ────────────────────────────────────────────────────────────

def "test parse-semver: parses simple version" [] {
    let v = (parse-semver "0.4.6")
    assert equal $v.major 0
    assert equal $v.minor 4
    assert equal $v.patch 6
}

def "test parse-semver: parses large numbers" [] {
    let v = (parse-semver "12.345.6789")
    assert equal $v.major 12
    assert equal $v.minor 345
    assert equal $v.patch 6789
}

def "test parse-semver: drops pre-release suffix" [] {
    let v = (parse-semver "1.2.3-rc.1")
    assert equal $v.major 1
    assert equal $v.minor 2
    assert equal $v.patch 3
}

def "test parse-semver: rejects invalid version" [] {
    let failed = (try { parse-semver "not-a-version"; false } catch { true })
    assert $failed
}

def "test parse-semver: rejects incomplete version" [] {
    let failed = (try { parse-semver "1.2"; false } catch { true })
    assert $failed
}

# ── next-patch ──────────────────────────────────────────────────────────────

def "test next-patch: bumps patch component" [] {
    assert equal (next-patch "0.4.6") "0.4.7"
}

def "test next-patch: keeps major and minor unchanged" [] {
    assert equal (next-patch "3.2.1") "3.2.2"
}

def "test next-patch: rolls over from zero" [] {
    assert equal (next-patch "0.0.0") "0.0.1"
}

def "test next-patch: handles double digit patch" [] {
    assert equal (next-patch "1.0.9") "1.0.10"
}

def "test next-patch: drops pre-release suffix on bump" [] {
    assert equal (next-patch "0.5.0-beta.2") "0.5.1"
}

def "test next-patch: matches Cargo.toml current version format" [] {
    let repo_root = ($env.CURRENT_FILE | path dirname | path dirname | path dirname)
    let cargo_version = (open ($repo_root | path join "Cargo.toml") | get package.version)
    let bumped = (next-patch $cargo_version)
    let is_semver = ($bumped | find --regex '^\d+\.\d+\.\d+$' | is-not-empty)
    assert $is_semver
}

# ── compare-semver ──────────────────────────────────────────────────────────

def "test compare-semver: equal versions" [] {
    assert equal (compare-semver "1.2.3" "1.2.3") 0
}

def "test compare-semver: lower major" [] {
    assert equal (compare-semver "1.0.0" "2.0.0") (-1)
}

def "test compare-semver: higher patch" [] {
    assert equal (compare-semver "1.2.5" "1.2.3") 1
}

def "test compare-semver: higher minor beats lower patch" [] {
    assert equal (compare-semver "1.3.0" "1.2.9") 1
}

# ── highest-tag-version ─────────────────────────────────────────────────────

def "test highest-tag-version: picks the max among several tags" [] {
    let tags = ["v0.4.5", "v0.4.6", "v0.4.1"]
    assert equal (highest-tag-version $tags) "0.4.6"
}

def "test highest-tag-version: ignores malformed tags" [] {
    let tags = ["v0.4.5", "not-a-tag", "latest"]
    assert equal (highest-tag-version $tags) "0.4.5"
}

def "test highest-tag-version: returns null for empty list" [] {
    assert equal (highest-tag-version []) null
}

def "test highest-tag-version: returns null when nothing matches" [] {
    assert equal (highest-tag-version ["foo", "bar"]) null
}

# ── safe-next-version ───────────────────────────────────────────────────────

def "test safe-next-version: bumps from Cargo.toml when tags are behind" [] {
    let tags = ["v0.1.0"]
    let result = (safe-next-version "0.4.6" "__tui-checkbox-nonexistent-crate__" $tags)
    assert equal $result "0.4.7"
}

def "test safe-next-version: prefers a git tag ahead of Cargo.toml, no downgrade" [] {
    let tags = ["v0.5.0"]
    let result = (safe-next-version "0.4.6" "__tui-checkbox-nonexistent-crate__" $tags)
    assert equal $result "0.5.1"
}

def "test safe-next-version: ignores tags behind Cargo.toml" [] {
    let tags = ["v0.1.0", "v0.2.0"]
    let result = (safe-next-version "1.0.0" "__tui-checkbox-nonexistent-crate__" $tags)
    assert equal $result "1.0.1"
}

def "test safe-next-version: handles no tags at all" [] {
    let result = (safe-next-version "0.4.6" "__tui-checkbox-nonexistent-crate__" [])
    assert equal $result "0.4.7"
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }
