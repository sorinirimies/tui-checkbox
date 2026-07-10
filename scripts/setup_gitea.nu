#!/usr/bin/env nu
# ──────────────────────────────────────────────────────────────────────────────
# tui-checkbox — Setup Gitea Remote
# ──────────────────────────────────────────────────────────────────────────────
# Adds or updates a Gitea remote and optionally pushes.
#
# Usage:
#   nu scripts/setup_gitea.nu <url>
#
# Examples:
#   nu scripts/setup_gitea.nu "ssh://git@192.168.1.204:30009/sorin/tui-checkbox.git"
#   nu scripts/setup_gitea.nu "gitea@192.168.1.44:sorin/tui-checkbox.git"
# ──────────────────────────────────────────────────────────────────────────────

def main [
    url: string,                      # Gitea repository URL (SSH format recommended)
    --remote-name: string = "gitea",  # Name for the git remote (default: gitea)
] {
    print $"(ansi cyan)══════════════════════════════════════════════════════════════(ansi reset)"
    print $"(ansi cyan)  tui-checkbox — Setup Gitea Remote(ansi reset)"
    print $"(ansi cyan)══════════════════════════════════════════════════════════════(ansi reset)"
    print ""

    # Check if we're in a git repository
    let git_check = (do { git rev-parse --git-dir } | complete)
    if $git_check.exit_code != 0 {
        print $"(ansi red)❌ Not a git repository. Run this from the tui-checkbox directory.(ansi reset)"
        exit 1
    }

    # Check if the remote already exists
    let remotes = (git remote | lines)
    if $remote_name in $remotes {
        print $"  ⚠️  Remote '($remote_name)' already exists — updating URL…"
        git remote set-url $remote_name $url
        print $"  (ansi green)✓(ansi reset) Remote URL updated."
    } else {
        print $"  Adding remote '($remote_name)'…"
        git remote add $remote_name $url
        print $"  (ansi green)✓(ansi reset) Remote '($remote_name)' added."
    }

    print ""
    print "  Current remotes:"
    git remote -v | lines | each { |l| print $"    ($l)" }
    print ""

    # Test connection
    print "  Testing connection…"
    let ls_result = (do { git ls-remote $remote_name } | complete)
    if $ls_result.exit_code == 0 {
        print $"  (ansi green)✓(ansi reset) Successfully connected to ($remote_name)!"
    } else {
        print $"  (ansi yellow)⚠(ansi reset) Could not connect to ($remote_name)."
        print "    This is normal if the repository doesn't exist yet on Gitea."
        print ""
        print "    To create the repository on Gitea:"
        print "      1. Log in to your Gitea instance"
        print "      2. Click '+' → New Repository"
        print "      3. Repository name: tui-checkbox"
        print "      4. Do NOT initialize with README"
        print "      5. Click 'Create Repository'"
        print $"      6. Then run: git push ($remote_name) main"
    }

    print ""
    print $"(ansi green)✓(ansi reset) Setup complete!"
    print ""
    print "  Quick commands:"
    print $"    git push ($remote_name) main        # Push to Gitea"
    print $"    git pull ($remote_name) main        # Pull from Gitea"
    print $"    just push-all                       # Push to all remotes"
    print ""
}
