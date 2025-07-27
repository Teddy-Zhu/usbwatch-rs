#!/bin/bash

# Version bump script for usbwatch-rs
# Usage: ./bump-version.sh [major|minor|patch] [new_version]
# Examples:
#   ./bump-version.sh patch        # Auto-increment patch version
#   ./bump-version.sh minor        # Auto-increment minor version  
#   ./bump-version.sh major        # Auto-increment major version
#   ./bump-version.sh 0.2.0        # Set specific version

set -e

CARGO_TOML="Cargo.toml"

if [ ! -f "$CARGO_TOML" ]; then
    echo "❌ Error: Cargo.toml not found in current directory"
    exit 1
fi

# Get current version
CURRENT_VERSION=$(grep '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/')
echo "📋 Current version: $CURRENT_VERSION"

if [ $# -eq 0 ]; then
    echo "📚 Usage: $0 [major|minor|patch|<version>]"
    echo "   $0 patch   # $CURRENT_VERSION -> increment patch"
    echo "   $0 minor   # $CURRENT_VERSION -> increment minor"  
    echo "   $0 major   # $CURRENT_VERSION -> increment major"
    echo "   $0 0.2.0   # $CURRENT_VERSION -> 0.2.0"
    exit 0
fi

BUMP_TYPE="$1"

# Parse current version
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

case "$BUMP_TYPE" in
    "major")
        NEW_VERSION="$((MAJOR + 1)).0.0"
        ;;
    "minor")
        NEW_VERSION="$MAJOR.$((MINOR + 1)).0"
        ;;
    "patch")
        NEW_VERSION="$MAJOR.$MINOR.$((PATCH + 1))"
        ;;
    *)
        # Assume it's a specific version
        if [[ "$BUMP_TYPE" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            NEW_VERSION="$BUMP_TYPE"
        else
            echo "❌ Error: Invalid version format. Use major|minor|patch or a version like 1.2.3"
            exit 1
        fi
        ;;
esac

echo "🚀 Bumping version: $CURRENT_VERSION -> $NEW_VERSION"

# Update Cargo.toml
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"

# Update Cargo.lock
cargo check > /dev/null 2>&1

echo "✅ Version updated successfully!"
echo "📝 Next steps:"
echo "   1. Review the changes: git diff"
echo "   2. Test the build: cargo build --release"
echo "   3. Commit the changes: git add -A && git commit -m \"chore: bump version to $NEW_VERSION\""
echo "   4. Tag the release: git tag v$NEW_VERSION"
echo "   5. Push with tags: git push origin main --tags"
