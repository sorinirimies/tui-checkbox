#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-checkbox — Idempotent crates.io publish
# ──────────────────────────────────────────────────────────────────────────────
# Publishes the current crate to crates.io, but only when needed:
#   • If the current version is already on crates.io → skip (exit 0).
#   • If CARGO_REGISTRY_TOKEN is missing → skip with a warning (exit 0).
#   • Otherwise → publish. A race where the version appears mid-publish
#     ("already exists") is treated as success.
#
# This makes re-running a release (e.g. re-pushing a tag) safe and non-fatal.
#
# Usage:
#   nu scripts/ci/publish.nu            # real, idempotent publish
#   nu scripts/ci/publish.nu --dry-run  # verify packaging only
#
# Auth:
#   Reads the token from the CARGO_REGISTRY_TOKEN environment variable
#   (the modern replacement for the deprecated `cargo publish --token` flag).
# ──────────────────────────────────────────────────────────────────────────────

# Sparse-index path for a crate name, per the crates.io index layout.
#   1 char   → 1/{name}
#   2 chars  → 2/{name}
#   3 chars  → 3/{first}/{name}
#   4+ chars → {first-2}/{next-2}/{name}
export def index-path [name: string]: nothing -> string {
    let n = ($name | str downcase)
    let chars = ($n | split chars)
    match ($chars | length) {
        1 => $"1/($n)"
        2 => $"2/($n)"
        3 => $"3/($chars | first 1 | str join)/($n)"
        _ => {
            let a = ($chars | first 2 | str join)
            let b = ($chars | skip 2 | first 2 | str join)
            $"($a)/($b)/($n)"
        }
    }
}

# Returns true if `version` of crate `name` is already published on crates.io.
# Network/404 failures are treated as "not published" (fail-open) so the caller
# can fall back to letting `cargo publish` be the source of truth.
export def is-published [name: string, version: string]: nothing -> bool {
    let url = $"https://index.crates.io/(index-path $name)"
    let body = (try { http get --raw $url | into string } catch { "" })
    if ($body | str trim | is-empty) {
        return false
    }
    $body
    | lines
    | where { |line| ($line | str trim | is-not-empty) }
    | any { |line| (($line | from json | get vers) == $version) }
}

def main [
    --dry-run,  # Package and verify without uploading
] {
    let name = (open Cargo.toml | get package.name)
    let version = (open Cargo.toml | get package.version)

    print $"(ansi cyan)📦 crates.io publish — ($name)@($version)(ansi reset)"

    if (is-published $name $version) {
        print $"(ansi yellow)⏭  ($name)@($version) is already on crates.io — skipping publish.(ansi reset)"
        return
    }

    if $dry_run {
        print "  Running `cargo publish --dry-run`…"
        cargo publish --dry-run
        print $"(ansi green)✅ Dry-run OK — ($name)@($version) is ready to publish.(ansi reset)"
        return
    }

    if ($env.CARGO_REGISTRY_TOKEN? | default "" | is-empty) {
        print $"(ansi yellow)⚠  CARGO_REGISTRY_TOKEN not set — skipping crates.io publish.(ansi reset)"
        print "   Set it in your shell or as the CRATES_IO_TOKEN CI secret to enable publishing."
        return
    }

    print $"(ansi green)🚀 Publishing ($name)@($version)…(ansi reset)"
    let result = (do { cargo publish } | complete)
    if ($result.stdout | is-not-empty) { print $result.stdout }

    if $result.exit_code != 0 {
        # Tolerate a race where the version was published between our check and now.
        if ($result.stderr | str contains "already exists") {
            print $"(ansi yellow)⏭  ($name)@($version) already exists — treating as success.(ansi reset)"
            return
        }
        print --stderr $result.stderr
        exit 1
    }

    print $"(ansi green)✅ ($name)@($version) published to crates.io.(ansi reset)"
}
