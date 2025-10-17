# SPDX-FileCopyrightText: Copyright (C) Nicolas Lamirault <nicolas.lamirault@gmail.com>
# SPDX-License-Identifier: Apache-2.0

# ltui Makefile

# Colors for terminal output
COLOR_RESET=\033[0m
COLOR_BLUE=\033[34m
COLOR_GREEN=\033[32m

# Directories
BIN_DIR ?= ./target/release
# Installation directory
PREFIX ?= ~/.local/bin

# Build settings
CARGO ?= cargo
CARGO_FLAGS ?=
RELEASE_FLAGS ?= --release
RUN_FLAGS ?=

help: ## Display this help message
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(COLOR_BLUE)%-20s$(COLOR_RESET) %s\n", $$1, $$2}'

build: ## Development builds
	$(CARGO) build $(CARGO_FLAGS)

release: ## Release builds
	$(CARGO) build $(RELEASE_FLAGS)

run: ## Run the application
	RUST_BACKTRACE=1 $(CARGO) run $(CARGO_FLAGS) $(RUN_FLAGS)

# Run with specific project
run-project:
	@echo "Enter project ID:"; \
	read PROJECT_ID; \
	RUST_BACKTRACE=1 $(CARGO) run $(CARGO_FLAGS) -- --project $$PROJECT_ID

# Clean build artifacts
clean:
	$(CARGO) clean
	rm -rf target/

# Check code without building
check:
	$(CARGO) check

# Run tests
test:
	$(CARGO) test $(CARGO_FLAGS)

# Lint the code
lint:
	$(CARGO) clippy -- -D warnings

# Format code
fmt:
	$(CARGO) fmt

# Generate documentation
docs:
	$(CARGO) doc --no-deps
	@echo "Documentation generated in target/doc"

# Install the application
install: release
	@mkdir -p $(PREFIX)
	@cp $(BIN_DIR)/ltui $(PREFIX)/
	@chmod +x $(PREFIX)/ltui
	@echo "Installed ltui to $(PREFIX)/ltui"

# Uninstall the application
uninstall:
	@rm -f $(PREFIX)/ltui
	@echo "Uninstalled ltui from $(PREFIX)/ltui"

# Check environment
check-env:
	@echo "Checking required dependencies..."
	@which gcloud > /dev/null || (echo "Error: gcloud CLI not found. Please install Google Cloud SDK." && exit 1)
	@gcloud auth list --filter=status:ACTIVE --format="value(account)" | grep -q "@" || echo "Warning: No active gcloud account found. Run 'gcloud auth login' to authenticate."
	@PROJECT=$$(gcloud config get-value project 2>/dev/null); \
	if [ -z "$$PROJECT" ] || [ "$$PROJECT" = "(unset)" ]; then \
		echo "Warning: No default project set. Run 'gcloud config set project PROJECT_ID'"; \
	else \
		echo "Using project: $$PROJECT"; \
	fi

# Help documentation
help:
	@echo "ltui - Linear TUI"
	@echo ""
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all         Build the development version (default)"
	@echo "  build       Build the development version"
	@echo "  release     Build the release version"
	@echo "  run         Run the development version"
	@echo "  run-project Run with a specific project ID (--project)"
	@echo "  clean       Remove build artifacts"
	@echo "  check       Check code without building"
	@echo "  test        Run tests"
	@echo "  lint        Run linter (clippy)"
	@echo "  fmt         Format code"
	@echo "  docs        Generate documentation"
	@echo "  install     Install the application to $(PREFIX)"
	@echo "  uninstall   Uninstall the application"
	@echo "  check-env   Check if environment is properly set up"
	@echo "  help        Show this help message"
	@echo ""
	@echo "Environment variables:"
	@echo "  CARGO_FLAGS    Additional flags for cargo"
	@echo "  RELEASE_FLAGS  Additional flags for release build"
	@echo "  RUN_FLAGS      Additional flags for run target"
	@echo "  PREFIX         Installation directory (default: ~/.local/bin)"
	@echo ""
	@echo "Command line options:"
	@echo "  --project, -p  Specify Google Cloud project ID"
	@echo "  --region, -g   Filter by Google Cloud region"
	@echo "  --refresh, -r  Set auto-refresh interval in seconds"
	@echo "  --config, -c   Path to config file"
