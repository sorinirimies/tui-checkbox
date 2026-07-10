#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-checkbox — Pre-publish checks
# ──────────────────────────────────────────────────────────────────────────────
# Runs formatting, clippy, tests, documentation, and dry-run publish checks
# before an actual `cargo publish`.
#
# Usage:
#   nu scripts/check_publish.nu
# ──────────────────────────────────────────────────────────────────────────────

def main [] {
    print "══════════════════════════════════════════════════════════"
    print "  tui-checkbox — Pre-publish checks"
    print "══════════════════════════════════════════════════════════"
    print ""

    mut errors = 0

    # ── 1. Formatting ─────────────────────────────────────────────────────
    print "── Step 1: Checking code formatting ──"
    let fmt_result = (do { cargo fmt -- --check } | complete)
    if $fmt_result.exit_code != 0 {
        print "  ❌ Code formatting check failed"
        print $fmt_result.stderr
        $errors = $errors + 1
    } else {
        print "  ✅ Code formatting OK"
    }
    print ""

    # ── 2. Clippy ─────────────────────────────────────────────────────────
    print "── Step 2: Running clippy ──"
    let clippy_result = (do { cargo clippy --lib -- -D warnings } | complete)
    if $clippy_result.exit_code != 0 {
        print "  ❌ Clippy found issues"
        print $clippy_result.stderr
        $errors = $errors + 1
    } else {
        print "  ✅ Clippy passed"
    }
    print ""

    # ── 3. Tests ──────────────────────────────────────────────────────────
    print "── Step 3: Running tests ──"
    let test_result = (do { cargo test --all-features } | complete)
    if $test_result.exit_code != 0 {
        print "  ❌ Tests failed"
        print $test_result.stderr
        $errors = $errors + 1
    } else {
        print "  ✅ All tests passed"
    }
    print ""

    # ── 4. Documentation ─────────────────────────────────────────────────
    print "── Step 4: Building documentation ──"
    let doc_result = (do { cargo doc --no-deps } | complete)
    if $doc_result.exit_code != 0 {
        print "  ❌ Documentation build failed"
        print $doc_result.stderr
        $errors = $errors + 1
    } else {
        print "  ✅ Documentation builds OK"
    }
    print ""

    # ── 5. Build examples ────────────────────────────────────────────────
    print "── Step 5: Building examples ──"
    let examples_result = (do { cargo build --examples } | complete)
    if $examples_result.exit_code != 0 {
        print "  ❌ Examples build failed"
        print $examples_result.stderr
        $errors = $errors + 1
    } else {
        print "  ✅ Examples build OK"
    }
    print ""

    # ── 6. Required files ────────────────────────────────────────────────
    print "── Step 6: Checking required files ──"
    let required_files = ["README.md" "LICENSE" "Cargo.toml" "CHANGELOG.md"]
    mut missing = 0
    for file in $required_files {
        if not ($file | path exists) {
            print $"  ❌ Missing: ($file)"
            $missing = $missing + 1
        }
    }
    if $missing > 0 {
        $errors = $errors + 1
    } else {
        print "  ✅ All required files present"
    }
    print ""

    # ── 7. Dry-run publish ───────────────────────────────────────────────
    print "── Step 7: Dry-run publish ──"
    let publish_result = (do { cargo publish --dry-run } | complete)
    if $publish_result.exit_code != 0 {
        print "  ❌ Publish dry-run failed"
        print $publish_result.stderr
        $errors = $errors + 1
    } else {
        print "  ✅ Publish dry-run OK"
    }
    print ""

    # ── Summary ───────────────────────────────────────────────────────────
    print "══════════════════════════════════════════════════════════"
    if $errors == 0 {
        print "  ✅ All pre-publish checks passed!"
        print ""
        print "  Next step:"
        print "    cargo publish"
    } else {
        print $"  ❌ ($errors) check\(s\) failed. Please fix before publishing."
        exit 1
    }
    print "══════════════════════════════════════════════════════════"
}
