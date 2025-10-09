# Publishing to crates.io

This guide explains how to publish this library to crates.io.

## Prerequisites

1. Create a crates.io account at https://crates.io/
2. Get your API token from https://crates.io/me
3. Login to cargo:
   ```bash
   cargo login <your-api-token>
   ```

## Pre-publish Checklist

- [x] All tests pass (`cargo test`)
- [x] Examples build and run (`cargo build --examples`)
- [x] Documentation builds (`cargo doc`)
- [x] Package builds (`cargo package`)
- [x] README.md is comprehensive
- [x] LICENSE file exists (MIT)
- [x] CHANGELOG.md is up to date
- [x] Cargo.toml metadata is complete
- [ ] Git repository is clean (commit all changes)
- [ ] Version number is correct in Cargo.toml

## Publishing Steps

1. **Commit all changes to git:**
   ```bash
   git add .
   git commit -m "Prepare v0.1.0 for release"
   git push
   ```

2. **Create a git tag:**
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```

3. **Publish to crates.io:**
   ```bash
   cargo publish
   ```

## After Publishing

1. Verify the package appears on crates.io:
   https://crates.io/crates/py-import-helper
2. Check the documentation on docs.rs: https://docs.rs/py-import-helper
3. Test installation in a new project:
   ```bash
   cargo new test-project
   cd test-project
   # Add py-import-helper = "0.1" to Cargo.toml
   cargo build
   ```

## Future Releases

For subsequent releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new changes
3. Update documentation if needed
4. Run all tests and examples
5. Commit, tag, and publish as above

## Troubleshooting

### Package verification failed
- Run `cargo package --list` to see what files will be included
- Check `.gitignore` to ensure required files aren't excluded
- Use `cargo package --allow-dirty` for testing (not for actual publishing)

### Documentation issues
- Test docs locally: `cargo doc --open`
- Fix any warnings or broken links
- Ensure all public APIs are documented

### Version conflicts
- Ensure version in `Cargo.toml` is higher than published version
- Use semantic versioning (MAJOR.MINOR.PATCH)
- Breaking changes require major version bump (when >= 1.0.0)

## Publishing Checklist

Before running `cargo publish`:

```bash
# 1. Clean build
cargo clean
cargo build --release

# 2. Run all tests
cargo test

# 3. Build examples
cargo build --examples

# 4. Build documentation
cargo doc --no-deps

# 5. Verify package
cargo package --list
cargo package

# 6. Publish (no going back!)
cargo publish
```

## Resources

- [The Cargo Book -
  Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Publishing Guidelines](https://crates.io/policies)
- [Semantic Versioning](https://semver.org/)
