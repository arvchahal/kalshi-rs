.PHONY: format format-check test build clippy help

# Default target
help:
	@echo "Available targets:"
	@echo "  format       - Format all Rust files with prettyplease"
	@echo "  format-check - Check if all files are formatted"
	@echo "  test         - Run all tests"
	@echo "  build        - Build the project"
	@echo "  clippy       - Run clippy linter"
	@echo "  check        - Run format-check, clippy, and tests"

format:
	@echo "Formatting all Rust files..."
	@prettyplease-fmt $$(find src tests examples -name "*.rs" 2>/dev/null)
	@echo "Formatting complete!"

# Check formatting without modifying files
format-check:
	@echo "Checking formatting..."
	@prettyplease-fmt --check $$(find src tests examples -name "*.rs" 2>/dev/null)

# Run tests
test:
	@echo "Running tests..."
	@cargo test

# Build project
build:
	@echo "Building project..."
	@cargo build

# Run clippy
clippy:
	@echo "Running clippy..."
	@cargo clippy -- -D warnings

# Run all checks (format, clippy, test)
check: format-check clippy test
	@echo " All checks passed!"
