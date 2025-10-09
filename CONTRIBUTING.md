# Contributing 🤝

Hey! Thanks for wanting to help out. I'm still learning Rust myself, so don't
worry if you're not an expert - we can figure things out together!

## How to contribute

**Found a bug or want a feature?** Open an issue first so we can chat about it.

**Want to submit code?** Here's the process:

1. Fork the repo
2. Create a branch: `git checkout -b fix-something-cool`
3. Make your changes
4. Run the tests: `cargo test`
5. Make sure it passes clippy: `cargo clippy --all-targets --all-features -- -D warnings`
6. Format the code: `cargo fmt`
7. Push and open a pull request

## What you'll need

- Rust 1.70+ (get it from [rustup.rs](https://rustup.rs/))
- That's it!

## Running stuff locally

```bash
# Run tests
cargo test

# Check for issues
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt

# Try the examples
cargo run --example basic_usage
```

## Types of help I'd love

- **Bug fixes** - especially edge cases with Python imports
- **Performance improvements** - I'm still learning optimal Rust patterns
- **Documentation** - making examples clearer
- **Tests** - more real-world Python import scenarios
- **Features** - if you have ideas for useful functionality

## Questions?

Just open an issue! No question is too basic.

## Code style

The CI will check formatting and clippy warnings, but the main things:

- Run `cargo fmt` before pushing
- Fix any clippy warnings
- Add tests for new features
- Keep functions reasonably small
- Use descriptive names

That's it! Looking forward to your contributions 🎉
