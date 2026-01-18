# Contributing to Rust SSR Blog

Thank you for your interest in contributing! This document provides guidelines for contributing to this project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Set up the development environment**:
   ```bash
   nix develop  # if using Nix
   # or install Rust via rustup
   ```
4. **Create a feature branch**: `git checkout -b feature/my-new-feature`

## Development Workflow

### Building

```bash
cargo build
```

### Running Tests

Always run tests before submitting a PR:

```bash
cargo test
```

### Code Quality

Ensure your code passes all quality checks:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check for compilation errors
cargo check
```

### Running the Server

```bash
# Development mode with hot reload
cargo watch -x run

# Or standard run
cargo run
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add comments for complex logic
- Write doc comments for public APIs

### Example

```rust
/// Parse a blog post from a markdown file
///
/// # Arguments
///
/// * `path` - Path to the markdown file
///
/// # Returns
///
/// Returns a `Post` struct or a `ContentError`
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use blog_content::load_post;
///
/// let post = load_post(Path::new("post.md"))?;
/// ```
pub fn load_post(path: &Path) -> Result<Post, ContentError> {
    // ...
}
```

## Testing Guidelines

### Unit Tests

- Write tests for all new functionality
- Test edge cases and error conditions
- Use descriptive test names

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_post() {
        // Arrange
        let content = "...";

        // Act
        let result = parse_post(content);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_frontmatter_returns_error() {
        let content = "invalid";
        let result = parse_post(content);
        assert!(result.is_err());
    }
}
```

### Integration Tests

For testing route handlers and full workflows, consider adding integration tests in `tests/` directory.

## Documentation

- Update README.md if adding new features
- Add doc comments to public APIs
- Update relevant documentation files

## Commit Messages

Write clear, descriptive commit messages:

```
Add table of contents generation for blog posts

- Implement TOC extraction from markdown headings
- Add rendering function for TOC HTML
- Include tests for slugification
- Update post template to display TOC

Closes #42
```

### Commit Message Format

```
<type>: <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

## Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new features
3. **Run all tests** and ensure they pass
4. **Run linting** and fix any issues
5. **Update CHANGELOG** if applicable
6. **Submit PR** with clear description

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe testing performed

## Checklist
- [ ] Tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Linter passes (`cargo clippy`)
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

## Adding New Features

### New Route Handlers

1. Add handler in `crates/blog-server/src/routes/`
2. Register route in `main.rs`
3. Add tests
4. Update documentation

### New Content Types

1. Add model in `crates/blog-content/src/models.rs`
2. Add parser logic in `parser.rs`
3. Add tests
4. Update templates if needed

### New Templates

1. Add template in `templates/`
2. Ensure it extends `base.html`
3. Test rendering
4. Add Tailwind classes

## Security

- Never commit secrets or credentials
- Validate all user input
- Use parameterized queries (if adding database)
- Follow security best practices
- Report security issues privately

## Questions?

Feel free to:
- Open an issue for discussion
- Ask questions in pull requests
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
