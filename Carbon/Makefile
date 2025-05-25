# Carbon Programming Language Makefile

.PHONY: all clean build test install

# Default target
all: build

# Build the Carbon interpreter
build:
	cd src && make

# Clean build artifacts
clean:
	cd src && make clean
	rm -f bin/carbon

# Install the binary to bin/
install: build
	cp src/carbon bin/carbon

# Run tests (placeholder for future test suite)
test: build
	@echo "Running Carbon tests..."
	@echo "Test suite not yet implemented"

# Development build with verbose output
dev:
	cd src && make clean && make

# Show help
help:
	@echo "Carbon Programming Language Build System"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Build the Carbon interpreter (default)"
	@echo "  build    - Build the Carbon interpreter"
	@echo "  clean    - Clean build artifacts"
	@echo "  install  - Build and install binary to bin/"
	@echo "  test     - Run test suite (not yet implemented)"
	@echo "  dev      - Development build with clean"
	@echo "  help     - Show this help message"
