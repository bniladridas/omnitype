# Release Scripts

This directory contains scripts to help with version management and releases.

## Scripts

### test.sh
Tests what the release workflow would generate locally without actually creating a release.

```bash
./scripts/test.sh
```

### bump.sh
Bumps the version in Cargo.toml and creates a git tag for release.

```bash
# Bump patch version (0.1.0 -> 0.1.1)
./scripts/bump.sh patch

# Bump minor version (0.1.0 -> 0.2.0)
./scripts/bump.sh minor

# Bump major version (0.1.0 -> 1.0.0)
./scripts/bump.sh major

# Set specific version
./scripts/bump.sh 1.2.3
```

After running the bump script, push the tag to trigger the release:
```bash
git push origin v1.2.3
```

## Release Process

1. Test locally: `./scripts/test.sh`
2. Bump version: `./scripts/bump.sh patch`
3. Push tag: `git push origin v0.1.1`
4. GitHub Actions will automatically:
   - Create a GitHub release with changelog
   - Build binaries for multiple platforms
   - Publish to crates.io (if configured)

## Version Numbers in Codebase

The following files contain version references:

- Cargo.toml - Main project version (automatically updated by bump script)
- README.md - Badge showing Rust version requirement
- .github/ISSUE_TEMPLATE/*.md - Example version numbers in templates
- CONTRIBUTING.md - Example version numbers
- SECURITY.md - References to patch versions