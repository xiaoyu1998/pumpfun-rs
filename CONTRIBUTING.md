# Contributing to PumpFun Rust SDK

Thank you for your interest in contributing to the PumpFun Rust SDK! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/pumpfun-rs.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

1. Install Rust and Cargo using [rustup](https://rustup.rs/)
2. Install Solana CLI tools following the [official guide](https://docs.solana.com/cli/install-solana-cli-tools)
3. Run `cargo build` to ensure everything compiles
4. Run `cargo test` to run the test suite

## Making Changes

1. Use conventional commit messages with emojis:
   - `✨ feat: add new feature`
   - `🐛 fix: fix bug in X`
   - `📝 docs: update README`
   - `♻️ refactor: refactor X component`
   - `🎨 style: format code`
   - `✅ test: add tests for X`
   - `⚡️ perf: improve performance`
   - `🌱 init: initial commit`
   - `🔧 chore: update dependencies`
2. Follow Rust coding conventions and style guidelines
3. Add tests for new functionality
4. Update documentation as needed
5. Ensure all tests pass: `cargo test`
6. Run `cargo fmt` to format code
7. Run `cargo clippy` to check for common mistakes

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Ensure your PR description clearly describes the problem and solution
3. Reference any related issues
4. Your PR will be reviewed by maintainers
5. Make requested changes if any
6. Once approved, your PR will be merged

## Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable and function names
- Document public APIs using rustdoc
- Keep functions focused and concise
- Use proper error handling

## Testing

- Write unit tests for new functionality
- Include integration tests where appropriate
- Test edge cases and error conditions
- Maintain test coverage

## Documentation

- Update API documentation for any changed functions
- Keep README and other documentation up to date
- Include examples for new features
- Document breaking changes

## Questions or Problems?

- Open an issue for bugs or feature requests
- Join our community channels for discussion
- Tag maintainers for urgent issues

## License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as specified in the repository.
