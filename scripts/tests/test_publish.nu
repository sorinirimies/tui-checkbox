#!/usr/bin/env nu
# ── tui-checkbox · test_publish.nu ──────────────────────────────────────────
# Tests for scripts/ci/publish.nu — crates.io sparse-index path derivation and
# (network-gated) publication checks.

use std/assert
use runner.nu *
use ../ci/publish.nu [index-path is-published]

# ── index-path: pure, offline ───────────────────────────────────────────────

def "test publish: index-path for the crate name" [] {
    assert equal (index-path "tui-checkbox") "tu/i-/tui-checkbox"
}

def "test publish: index-path one-char name" [] {
    assert equal (index-path "a") "1/a"
}

def "test publish: index-path two-char name" [] {
    assert equal (index-path "ab") "2/ab"
}

def "test publish: index-path three-char name" [] {
    assert equal (index-path "abc") "3/a/abc"
}

def "test publish: index-path four-char name" [] {
    assert equal (index-path "serde") "se/rd/serde"
}

def "test publish: index-path lowercases the name" [] {
    assert equal (index-path "Tui-Checkbox") "tu/i-/tui-checkbox"
}

def "test publish: index-path is deterministic" [] {
    assert equal (index-path "ratatui") (index-path "ratatui")
}

# ── is-published: network-gated ─────────────────────────────────────────────
# These reach out to crates.io. If offline, they are skipped (treated as pass)
# so the suite stays green in sandboxed environments.

def is-online []: nothing -> bool {
    (try { http get --raw "https://index.crates.io/config.json" | into string | is-not-empty } catch { false })
}

def "test publish: a released version reports published" [] {
    if not (is-online) { return }
    assert (is-published "tui-checkbox" "0.4.4")
}

def "test publish: an impossible version reports unpublished" [] {
    if not (is-online) { return }
    assert (not (is-published "tui-checkbox" "99.99.99"))
}

def "test publish: unknown crate reports unpublished" [] {
    if not (is-online) { return }
    assert (not (is-published "this-crate-should-never-exist-xyzzy-42" "1.0.0"))
}

# ── Main ────────────────────────────────────────────────────────────────────

def main [] { run-tests }
