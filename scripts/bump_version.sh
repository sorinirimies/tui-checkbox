#!/bin/bash

set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.2.0"
  exit 1
fi

VERSION=$1

echo "Bumping version to $VERSION"

# Update Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Generate updated changelog
if command -v git-cliff > /dev/null 2>&1; then
  git-cliff --tag v$VERSION --output CHANGELOG.md
  echo "✅ Changelog updated"
else
  echo "⚠️  git-cliff not found. Skipping changelog generation."
  echo "   Install with: cargo install git-cliff"
fi

# Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to $VERSION"

# Create tag
git tag -a "v$VERSION" -m "Release v$VERSION"

echo "✅ Version bumped to $VERSION"
echo "   - Cargo.toml updated"
echo "   - CHANGELOG.md updated"
echo "   - Commit created"
echo "   - Tag v$VERSION created"
echo ""
echo "Next steps:"
echo "  1. Review changes: git show"
echo "  2. Push: git push origin main"
echo "  3. Push tag: git push origin v$VERSION"
echo ""
echo "Or use: just release $VERSION"
