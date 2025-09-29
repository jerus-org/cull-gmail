# Contributing to cull-gmail

Thank you for your interest in contributing to cull-gmail! We welcome contributions from the community and appreciate your help in making this project better.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue on GitHub with:
- A clear, descriptive title
- Steps to reproduce the issue
- Expected behaviour vs actual behaviour
- Your environment (OS, Rust version, etc.)
- Any relevant error messages or logs

### Suggesting Features

Feature requests are welcome! Please open an issue describing:
- The problem you're trying to solve
- Your proposed solution
- Any alternative solutions you've considered

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes**, following the coding standards below
3. **Add tests** if applicable
4. **Ensure all tests pass** by running `cargo test`
5. **Update documentation** as needed
6. **Format your code** with `cargo fmt`
7. **Run the linter** with `cargo clippy`
8. **Commit your changes** with clear, descriptive commit messages
9. **Submit a pull request** with a description of your changes

## Development Setup

```bash
# Clone your fork
git clone https://github.com/your-username/project-name.git
cd project-name

# Build the project
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Coding Standards

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` to format code
- Ensure `cargo clippy` passes without warnings
- Write clear, self-documenting code with appropriate comments
- Add documentation comments for public APIs
- Keep functions focused and reasonably sized
- Write tests for new functionality

## Commit Messages

- Use clear, descriptive commit messages
- Use conventional commits 
- Start with a verb in the present tense (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Add a detailed description if necessary

## Testing

- Write unit tests for new functionality
- Ensure all existing tests pass
- Aim for meaningful test coverage
- Test edge cases and error conditions

## Documentation

- Update the README if your changes affect usage
- Add inline documentation for public APIs
- Include examples in doc comments where helpful

## PRLOG and CHANGELOG

 - The Pull Request log is updated automatically by CI.
 - The Changelog is generated automatically from relevant conventional commits by CI

## License

By contributing to cull-gmail, you agree that your contributions will be licensed under the MIT License. This means:

- You grant permission for your contributions to be used, modified, and distributed under the terms of the MIT License
- You confirm that you have the right to submit the code under this license
- You understand that your contributions will become part of the project and available to all users under the MIT License

## Questions?

If you have questions about contributing, feel free to open an issue or reach out to the maintainers.

Thank you for contributing!