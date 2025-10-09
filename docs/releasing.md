# Release Process

This project uses [release-plz](https://crates.io/crates/release-plz) for
automated releases.

## How It Works

1. **Automatic**: When you push commits to `main`, release-plz automatically:
   - Analyzes commits since the last release
   - Determines the appropriate version bump (patch/minor/major)
   - Creates a PR with version updates and changelog
   - When the PR is merged, creates a GitHub release and publishes to crates.io

2. **Manual Trigger**: You can also trigger the release workflow manually from
   the GitHub Actions tab.

## Commit Conventions

To get the most out of automatic changelog generation, use conventional commits:

- `feat:` - New features (minor version bump)
- `fix:` - Bug fixes (patch version bump)
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `test:` - Test changes
- `chore:` - Maintenance tasks
- `BREAKING CHANGE:` - Breaking changes (major version bump)

## Examples

```bash
# Patch version bump (0.2.0 -> 0.2.1)
git commit -m "fix: handle edge case in import parsing"

# Minor version bump (0.2.0 -> 0.3.0)
git commit -m "feat: add support for Python 3.14 imports"

# Major version bump (0.2.0 -> 1.0.0)
git commit -m "feat: new API design

BREAKING CHANGE: ImportHelper constructor now requires package_name parameter"
```

## Configuration

Release settings are configured in `release-plz.toml`:

- Changelog generation is enabled
- GitHub releases are created automatically
- Crates.io publishing is enabled
- Follows semantic versioning

## Manual Release (if needed)

If you ever need to create a release manually:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit: `git commit -m "chore: release v0.2.1"`
4. Tag: `git tag v0.2.1`
5. Push: `git push origin main --tags`

## Secrets Required

Make sure these GitHub secrets are configured:

- `GITHUB_TOKEN` - Automatically provided
- `CARGO_REGISTRY_TOKEN` - Your crates.io API token
