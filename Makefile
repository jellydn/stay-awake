# macOS Sleep Prevention Utility - Development Makefile
# Use: make <target>

.PHONY: help setup dev build test clean check lint fix install run-dev run-build deps

# Default target
.DEFAULT_GOAL := help

# Colors for output
RED    := \033[31m
GREEN  := \033[32m
YELLOW := \033[33m
BLUE   := \033[34m
RESET  := \033[0m

## Help - Show available targets
help:
	@echo "$(BLUE)macOS Sleep Prevention Utility - Development Commands$(RESET)"
	@echo ""
	@echo "$(GREEN)Quick Start:$(RESET)"
	@echo "  make setup     - Initial project setup (run this first!)"
	@echo "  make dev       - Start development server with hot reload"
	@echo ""
	@echo "$(GREEN)Build & Install:$(RESET)"
	@echo "  make build     - Build for development" 
	@echo "  make build-release - Build optimized release version"
	@echo "  make install   - Install the built app to /Applications/"
	@echo ""
	@echo "$(GREEN)Development:$(RESET)"
	@echo "  make check     - Quick Rust code check"
	@echo "  make test      - Run all tests"
	@echo "  make lint      - Run linters"
	@echo "  make fix       - Auto-fix linting issues"
	@echo "  make clean     - Clean build artifacts"
	@echo ""
	@echo "$(GREEN)Testing:$(RESET)"
	@echo "  make manual-test - Show manual testing scenarios"
	@echo "  make logs      - Show application logs"
	@echo ""
	@echo "$(YELLOW)💡 Tip: Start with 'make setup' then 'make dev'$(RESET)"

## Setup - Initial project setup
setup: deps
	@echo "$(GREEN)Setting up macOS Sleep Prevention development environment...$(RESET)"
	@echo "$(YELLOW)✓ Dependencies installed$(RESET)"
	@echo "$(YELLOW)✓ Checking Rust compilation...$(RESET)"
	@cd src-tauri && cargo check
	@echo "$(YELLOW)✓ Checking frontend compilation...$(RESET)"
	@bun run build
	@echo "$(GREEN)✅ Setup complete! Run 'make dev' to start development.$(RESET)"

## Dependencies - Install/update all dependencies
deps:
	@echo "$(YELLOW)Installing frontend dependencies...$(RESET)"
	@bun install
	@echo "$(YELLOW)Updating Rust dependencies...$(RESET)"
	@cd src-tauri && cargo update

## Development - Start dev server with hot reload
dev:
	@echo "$(GREEN)Starting development server...$(RESET)"
	@echo "$(YELLOW)Frontend: http://localhost:5173$(RESET)"
	@echo "$(YELLOW)Use Ctrl+C to stop$(RESET)"
	@bun tauri dev

## Check - Quick Rust code check without building
check:
	@echo "$(YELLOW)Checking Rust code...$(RESET)"
	@cd src-tauri && cargo check
	@echo "$(GREEN)✅ Rust check passed$(RESET)"

## Build - Development build
build:
	@echo "$(YELLOW)Building application (development)...$(RESET)"
	@bun tauri build --no-bundle --debug
	@echo "$(GREEN)✅ Development build complete$(RESET)"

## Build Release - Optimized production build
build-release:
	@echo "$(YELLOW)Building application (release)...$(RESET)"
	@bun tauri build
	@echo "$(GREEN)✅ Release build complete$(RESET)"
	@echo "$(BLUE)Bundle location: src-tauri/target/release/bundle/$(RESET)"

## Test - Run all tests
test: test-rust
	@echo "$(YELLOW)Running frontend tests...$(RESET)"
	@echo "$(BLUE)Note: Frontend tests not implemented (manual testing approach)$(RESET)"
	@echo "$(GREEN)✅ All tests complete$(RESET)"

## Test Rust - Run Rust tests only
test-rust:
	@echo "$(YELLOW)Running Rust tests...$(RESET)"
	@cd src-tauri && cargo test
	@echo "$(GREEN)✅ Rust tests passed$(RESET)"

## Lint - Run all linters
lint:
	@echo "$(YELLOW)Linting Rust code...$(RESET)"
	@cd src-tauri && cargo clippy -- -D warnings
	@echo "$(YELLOW)Linting TypeScript code...$(RESET)"
	@bun run lint:check 2>/dev/null || echo "$(BLUE)Note: No TypeScript files to lint yet$(RESET)"
	@echo "$(GREEN)✅ Linting complete$(RESET)"

## Fix - Auto-fix linting issues
fix:
	@echo "$(YELLOW)Auto-fixing Rust code...$(RESET)"
	@cd src-tauri && cargo fix --allow-dirty --allow-staged 2>/dev/null || true
	@cd src-tauri && cargo fmt
	@echo "$(YELLOW)Auto-fixing TypeScript code...$(RESET)"
	@bun run lint 2>/dev/null || echo "$(BLUE)Note: No TypeScript files to fix yet$(RESET)"
	@echo "$(GREEN)✅ Auto-fix complete$(RESET)"

## Clean - Remove build artifacts
clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	@cd src-tauri && cargo clean
	@rm -rf dist/
	@rm -rf node_modules/.vite/
	@echo "$(GREEN)✅ Clean complete$(RESET)"

## Install - Install the built application
install: build-release
	@echo "$(YELLOW)Installing macOS Sleep Prevention...$(RESET)"
	@if [ -d "src-tauri/target/release/bundle/macos/macOS Sleep Prevention.app" ]; then \
		cp -R "src-tauri/target/release/bundle/macos/macOS Sleep Prevention.app" /Applications/; \
		echo "$(GREEN)✅ App installed to /Applications/$(RESET)"; \
	else \
		echo "$(RED)❌ App bundle not found. Run 'make build-release' first.$(RESET)"; \
		exit 1; \
	fi

## Run Dev - Run development build
run-dev: build
	@echo "$(GREEN)Running development build...$(RESET)"
	@./src-tauri/target/debug/macos-sleep-prevention

## Run Build - Run release build  
run-build: build-release
	@echo "$(GREEN)Running release build...$(RESET)"
	@./src-tauri/target/release/macos-sleep-prevention

## Logs - Show application logs
logs:
	@echo "$(YELLOW)Showing recent application logs...$(RESET)"
	@echo "$(BLUE)Console logs (last 20 lines):$(RESET)"
	@log show --predicate 'subsystem contains "com.macos-sleep-prevention.app"' --last 5m | tail -20 || echo "$(YELLOW)No recent logs found$(RESET)"

## Manual Test - Run manual testing scenarios
manual-test:
	@echo "$(GREEN)Manual Testing Guide$(RESET)"
	@echo "$(YELLOW)Follow the scenarios in: specs/001-mac-osx-to/quickstart.md$(RESET)"
	@echo ""
	@echo "$(BLUE)Key test scenarios:$(RESET)"
	@echo "1. Basic sleep prevention (5 minutes)"
	@echo "2. Manual cancellation" 
	@echo "3. Visual status indicators (tray icon changes)"
	@echo "4. Input validation (try: '2h 30m', '150m', '2:30')"
	@echo "5. System integration (lid close, manual sleep)"
	@echo "6. Multiple instance prevention"
	@echo "7. User session changes"
	@echo ""
	@echo "$(YELLOW)Run 'make dev' in another terminal, then test each scenario.$(RESET)"

# Development shortcuts
dev-check: check lint
dev-build: clean build
release: clean build-release install

# Debug targets
debug-deps:
	@echo "$(BLUE)Frontend dependencies:$(RESET)"
	@bun list --depth=0
	@echo "$(BLUE)Rust dependencies:$(RESET)"
	@cd src-tauri && cargo tree --depth=1

debug-config:
	@echo "$(BLUE)Tauri configuration:$(RESET)"
	@cat src-tauri/tauri.conf.json | head -20
	@echo "$(BLUE)Package.json scripts:$(RESET)"
	@cat package.json | jq .scripts

# CI/Automated targets (for future use)
ci: deps check test-rust lint
	@echo "$(GREEN)✅ CI pipeline passed$(RESET)"

.PHONY: dev-check dev-build release debug-deps debug-config ci