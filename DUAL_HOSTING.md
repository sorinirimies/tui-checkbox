# Dual Hosting Strategy: GitHub + Gitea

This guide explains how to host tui-checkbox on both GitHub and your own Gitea instance.

## Overview

**Strategy:** Dual Primary with Automatic Sync

- **GitHub** - Primary public repository (for discoverability, crates.io, community)
- **Gitea** - Secondary/backup repository (for self-hosting, control, redundancy)

Both repositories are kept in sync automatically.

## Benefits

- ✅ **Redundancy** - Multiple hosting locations
- ✅ **Control** - Own your code on your infrastructure
- ✅ **Visibility** - Public on GitHub for discoverability
- ✅ **Flexibility** - Can switch primary at any time
- ✅ **CI/CD** - Run workflows on both platforms
- ✅ **Backup** - Automatic offsite backup

## Setup

### 1. Add Gitea Remote

Add your Gitea instance as a second remote:

```bash
# Add Gitea remote using justfile
just setup-gitea git@gitea.yourdomain.com:username/tui-checkbox.git

# Or add manually with SSH (recommended)
git remote add gitea git@gitea.yourdomain.com:username/tui-checkbox.git

# Or with HTTPS
git remote add gitea https://gitea.yourdomain.com/username/tui-checkbox.git

# Verify remotes
just remotes
```

You should see:

```
origin  git@github.com:sorinirimies/tui-checkbox.git (fetch)
origin  git@github.com:sorinirimies/tui-checkbox.git (push)
gitea   git@gitea.yourdomain.com:username/tui-checkbox.git (fetch)
gitea   git@gitea.yourdomain.com:username/tui-checkbox.git (push)
```

### 2. Initial Push to Gitea

```bash
# Push all branches and tags to Gitea
git push gitea --all
git push gitea --tags
```

### 3. Configure Push to Both Remotes

**Option A: Push to Both with One Command**

Add Gitea as an additional push URL for origin:

```bash
git remote set-url --add --push origin git@gitea.yourdomain.com:username/tui-checkbox.git
```

Now `git push origin` will push to both GitHub and Gitea!

**Option B: Create an "all" Remote**

```bash
git remote add all git@github.com:sorinirimies/tui-checkbox.git
git remote set-url --add --push all git@github.com:sorinirimies/tui-checkbox.git
git remote set-url --add --push all git@gitea.yourdomain.com:username/tui-checkbox.git

# Push to both
git push all
git push all --tags
```

### 4. Justfile Commands (Already Configured!)

Your justfile already includes all the necessary commands for dual hosting:

#### Basic Push Commands

```bash
# Push to GitHub only
just push

# Push to Gitea only  
just push-gitea

# Push to both GitHub and Gitea
just push-all
```

#### Tag Push Commands

```bash
# Push tags to GitHub only
just push-tags

# Push tags to both remotes
just push-tags-all
```

#### Release Workflows with Version Bumping

The justfile includes three release workflows:

**Option 1: Release to GitHub Only**
```bash
just release 0.2.0
```
This will:
1. Bump version to 0.2.0 in Cargo.toml
2. Generate changelog with git-cliff
3. Create git tag v0.2.0
4. Push to GitHub (main + tag)

**Option 2: Release to Gitea Only**
```bash
just release-gitea 0.2.0
```
This will:
1. Bump version to 0.2.0 in Cargo.toml
2. Generate changelog with git-cliff
3. Create git tag v0.2.0
4. Push to Gitea (main + tag)

**Option 3: Release to Both (Recommended)**
```bash
just release-all 0.2.0
```
This will:
1. Bump version to 0.2.0 in Cargo.toml
2. Generate changelog with git-cliff
3. Create git tag v0.2.0
4. Push to **both** GitHub and Gitea (main + tag)

#### Other Useful Commands

```bash
# Push existing release to both remotes (without bumping)
just push-release-all

# Sync Gitea with GitHub (force push)
just sync-gitea

# Show configured remotes
just remotes

# Setup Gitea remote
just setup-gitea git@gitea.yourdomain.com:username/tui-checkbox.git
```

The version bumping script (`scripts/bump_version.sh`) handles:
- Updating version in Cargo.toml
- Updating Cargo.lock
- Generating changelog with git-cliff
- Creating git commit and tag
- All changes are committed automatically

## CI/CD for Gitea

### Gitea Actions (GitHub Actions Compatible)

If your Gitea instance has Gitea Actions enabled (Gitea 1.19+), you can use the same workflows!

Create `.gitea/workflows/` (note: `.gitea` not `.github`):

```bash
mkdir -p .gitea/workflows
cp .github/workflows/ci.yml .gitea/workflows/ci.yml
cp .github/workflows/release.yml .gitea/workflows/release.yml
```

**Modify for Gitea:**

1. Update secrets names if needed
2. Adjust any GitHub-specific actions
3. Configure Gitea Actions runners

### Drone CI (Alternative)

If using Drone CI with Gitea:

Create `.drone.yml`:

```yaml
kind: pipeline
type: docker
name: default

steps:
  - name: format
    image: rust:latest
    commands:
      - cargo fmt -- --check

  - name: clippy
    image: rust:latest
    commands:
      - cargo clippy -- -D warnings

  - name: test
    image: rust:latest
    commands:
      - cargo test --all-features

  - name: build
    image: rust:latest
    commands:
      - cargo build --release

trigger:
  branch:
    - main
    - develop
  event:
    - push
    - pull_request
```

### Woodpecker CI (Alternative)

If using Woodpecker CI:

Create `.woodpecker.yml`:

```yaml
pipeline:
  format:
    image: rust:latest
    commands:
      - cargo fmt -- --check

  clippy:
    image: rust:latest
    commands:
      - cargo clippy -- -D warnings

  test:
    image: rust:latest
    commands:
      - cargo test --all-features

  build:
    image: rust:latest
    commands:
      - cargo build --release
```

## Daily Workflow

### Regular Development

```bash
# Make changes
git add .
git commit -m "feat: add new feature"

# Push to both remotes
just push-all

# Or use git directly
git push origin main
git push gitea main
```

### Creating a Release

```bash
# Release to GitHub only
just release 0.2.0

# Release to Gitea only
just release-gitea 0.2.0

# Release to BOTH GitHub and Gitea (recommended)
just release-all 0.2.0

# This will automatically:
# 1. Bump version in Cargo.toml
# 2. Update Cargo.lock
# 3. Generate changelog with git-cliff
# 4. Create commit and tag
# 5. Push to the selected remote(s)
```

**Before releasing, run checks:**
```bash
# Run all checks (format, clippy, tests)
just check-all

# Or run full release check
just release-check
```

### Pull from Either Remote

```bash
# Pull from GitHub (default)
git pull origin main

# Pull from Gitea
git pull gitea main

# Fetch from both
git fetch --all
```

## Sync Strategies

### Strategy 1: GitHub as Primary (Recommended)

**Use Case:** Public project, want GitHub visibility for crates.io

```bash
# Always push to GitHub first
git push origin main

# Then sync to Gitea
git push gitea main

# Or use the all remote
git push all main
```

**Benefits:**
- GitHub is the source of truth
- Gitea serves as backup/mirror
- Crates.io releases work seamlessly

### Strategy 2: Gitea as Primary

**Use Case:** Private development, push to GitHub for releases only

```bash
# Daily work pushes to Gitea
git push gitea main

# When ready to release, push to GitHub
git push origin main
git push origin --tags
```

**Benefits:**
- Keep development private on your server
- Control when code goes public
- Use Gitea for team collaboration

### Strategy 3: Equal Primary (Recommended Setup)

**Use Case:** Maximum redundancy and flexibility

```bash
# Daily development - push to both
just push-all

# For releases - push to both with version bump
just release-all 0.2.0
```

**Benefits:**
- Full redundancy
- Either can be primary if needed
- No single point of failure
- Both remotes always in sync

## Automatic Mirroring

### Option A: Git Hook (Local)

Create `.git/hooks/post-push`:

```bash
#!/bin/bash
# Automatically push to Gitea after pushing to GitHub

REMOTE_GITEA="gitea"

# Get current branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)

echo "Syncing to Gitea..."
git push $REMOTE_GITEA $BRANCH

# Also push tags
git push $REMOTE_GITEA --tags 2>/dev/null

echo "✅ Synced to Gitea!"
```

Make it executable:

```bash
chmod +x .git/hooks/post-push
```

### Option B: Gitea Repository Mirroring

In Gitea UI:
1. Go to Repository Settings
2. Enable "Mirror Repository"
3. Set GitHub as source
4. Configure sync interval

This pulls from GitHub automatically at intervals.

### Option C: GitHub Action to Push to Gitea

Create `.github/workflows/sync-gitea.yml`:

```yaml
name: Sync to Gitea

on:
  push:
    branches: [main, develop]
  release:
    types: [published]

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Push to Gitea
        env:
          GITEA_URL: ${{ secrets.GITEA_URL }}
          GITEA_TOKEN: ${{ secrets.GITEA_TOKEN }}
        run: |
          git remote add gitea $GITEA_URL
          git push gitea main --force
          git push gitea --tags --force
```

Add secrets to GitHub:
- `GITEA_URL`: Your Gitea repository URL
- `GITEA_TOKEN`: Gitea access token

## README Badges

Update your README.md to show both hosting locations:

```markdown
[![GitHub](https://img.shields.io/badge/GitHub-sorinirimies%2Ftui--checkbox-blue?logo=github)](https://github.com/sorinirimies/tui-checkbox)
[![Gitea](https://img.shields.io/badge/Gitea-self--hosted-green?logo=gitea)](https://gitea.yourdomain.com/username/tui-checkbox)

## 🏠 Hosting

This project is hosted on multiple platforms for redundancy:

- 🐙 **GitHub** - [sorinirimies/tui-checkbox](https://github.com/sorinirimies/tui-checkbox)
  - Primary public repository
  - CI/CD with GitHub Actions
  - Issue tracking and discussions
  
- 🍵 **Gitea** - [username/tui-checkbox](https://gitea.yourdomain.com/username/tui-checkbox)
  - Self-hosted mirror
  - CI/CD with Gitea Actions
  - Full backup and redundancy

Both repositories are kept in sync automatically. See [DUAL_HOSTING.md](DUAL_HOSTING.md) for details.
```

## Troubleshooting

### Push Fails to Gitea

```bash
# Check remote configuration
git remote -v

# Test connection
ssh -T git@gitea.yourdomain.com

# Force push if needed (be careful!)
git push gitea main --force
```

### Remotes Out of Sync

```bash
# Fetch from both
git fetch --all

# Compare branches
git log origin/main..gitea/main

# Sync Gitea with GitHub
git push gitea origin/main:main --force
```

### Authentication Issues

**For SSH:**
```bash
# Add SSH key to Gitea
cat ~/.ssh/id_rsa.pub

# Add to Gitea UI: Settings → SSH/GPG Keys
```

**For HTTPS:**
```bash
# Store credentials
git config credential.helper store

# Or use token
git remote set-url gitea https://username:token@gitea.yourdomain.com/username/tui-slider.git
```

## Best Practices

1. **Always push to both remotes** - Use justfile commands or git hooks
2. **Keep remotes in sync** - Don't let them diverge
3. **Use GitHub for releases** - For crates.io integration and visibility
4. **Use Gitea for backups** - Automatic redundancy
5. **Document URLs** - Update README with both repository links
6. **Test both CIs** - Ensure workflows work on both platforms
7. **Tag consistently** - Push tags to both remotes

## Security Considerations

1. **Gitea Access Control**
   - Set appropriate permissions
   - Use SSH keys for authentication
   - Enable 2FA if available

2. **Secrets Management**
   - Keep `CRATES_IO_TOKEN` only on GitHub
   - Use separate tokens for each platform
   - Never commit credentials

3. **Private vs Public**
   - GitHub: Usually public
   - Gitea: Can be private for development
   - Be careful what you push where

## Migration Plans

### If GitHub Goes Down

```bash
# Switch primary to Gitea
git remote rename origin github
git remote rename gitea origin

# Update URLs in documentation
# Continue development
```

### If Gitea Goes Down

```bash
# No action needed
# GitHub is still operational
# Continue development normally

# Re-sync after restoration using justfile
just sync-gitea

# Or manually
git push gitea --all
git push gitea --tags
```

## Quick Reference Commands

### Initial Setup
```bash
# Add Gitea remote using justfile
just setup-gitea git@gitea.yourdomain.com:username/tui-checkbox.git

# Or manually
git remote add gitea git@gitea.yourdomain.com:username/tui-checkbox.git

# Initial push to Gitea
git push gitea --all
git push gitea --tags
```

### Daily Development
```bash
# Push changes to both remotes
just push-all

# Commit and push
just commit "feat: add new feature"
just push-all
```

### Creating Releases
```bash
# Release to both GitHub and Gitea (recommended)
just release-all 0.2.0

# Release to GitHub only
just release 0.2.0

# Release to Gitea only
just release-gitea 0.2.0
```

### Maintenance
```bash
# Check remotes status
just remotes

# Sync Gitea with GitHub (force)
just sync-gitea

# Push tags to both
just push-tags-all
```

### Pre-Release Checks
```bash
# Run all checks
just check-all

# Run full release check (format, clippy, test, build)
just release-check
```

## Resources

- [Gitea Documentation](https://docs.gitea.io/)
- [Gitea Actions](https://docs.gitea.io/en-us/usage/actions/overview/)
- [Git Remote Documentation](https://git-scm.com/docs/git-remote)
- [Multiple Remote Workflows](https://git-scm.com/book/en/v2/Git-Basics-Working-with-Remotes)