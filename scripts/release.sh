#!/usr/bin/env bash
set -euo pipefail

# 1-Click Release Script for Livediff
# Usage: ./scripts/release.sh <new_version> (e.g., ./scripts/release.sh 3.2.1)

if [ $# -lt 1 ]; then
  echo "Error: Version argument required."
  echo "Usage: $0 <version> (e.g. $0 3.2.1)"
  exit 1
fi

VERSION="$1"
TAG="v${VERSION#v}"

echo "==> Preparing release ${TAG} for Livediff..."

# Check clean git state
if [ -n "$(git status --porcelain)" ]; then
  echo "Error: Working directory is not clean. Commit or stash changes first."
  exit 1
fi

# Run tests
echo "==> Running test suite..."
cargo test --all

# Update Cargo.toml
echo "==> Updating Cargo.toml version to ${VERSION}..."
sed -i.bak -E "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
rm -f Cargo.toml.bak
cargo check

# Commit version bump
git add Cargo.toml Cargo.lock
git commit -m "chore(release): bump version to ${TAG}"

# Tag release
git tag -a "${TAG}" -m "Release ${TAG}"

echo "==> Pushing commit and tag ${TAG} to origin/main..."
git push origin main
git push origin "${TAG}"

echo "✓ Release ${TAG} triggered successfully on GitHub Actions!"
echo "Track pipeline at: https://github.com/SoCkEt7/Livediff/actions"
