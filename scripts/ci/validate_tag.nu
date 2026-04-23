#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-checkbox — Validate release tag
# ──────────────────────────────────────────────────────────────────────────────
# Validates that a tag matches the vX.Y.Z pattern and outputs the tag and the
# bare version string.
#
# Usage:
#   nu scripts/ci/validate_tag.nu <tag>
#
# Examples:
#   nu scripts/ci/validate_tag.nu v0.5.0
#   nu scripts/ci/validate_tag.nu v1.2.3
# ──────────────────────────────────────────────────────────────────────────────

# Validate a tag string and return the tag + bare version.
export def validate [tag: string]: nothing -> record<tag: string, version: string> {
    if ($tag | is-empty) {
        error make {
            msg: $"Tag is empty — nothing to validate."
        }
    }

    let pattern = '^v\d+\.\d+\.\d+$'
    if ($tag | find --regex $pattern | is-empty) {
        error make {
            msg: $"Tag '($tag)' does not match vX.Y.Z — aborting."
        }
    }

    let version = ($tag | str replace 'v' '')

    { tag: $tag, version: $version }
}

def main [tag: string] {
    let result = try {
        validate $tag
    } catch { |err|
        print --stderr $"(ansi red)❌ ($err.msg)(ansi reset)"
        exit 1
    }

    print --stderr $"(ansi green)✅ Tag ($result.tag) \(version ($result.version)\) is valid.(ansi reset)"

    print $"tag=($result.tag)"
    print $"version=($result.version)"
}
