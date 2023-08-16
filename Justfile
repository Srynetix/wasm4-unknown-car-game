CART_TITLE := "Unknown Car Game"
CART_DESCRIPTION := "Unknown Car Game"

CART_FILE := "cart.wasm"
CART_DEBUG_PATH := "./target/wasm32-unknown-unknown/debug" / CART_FILE
CART_RELEASE_PATH := "./target/wasm32-unknown-unknown/release" / CART_FILE

WEB_EXPORT_RELEASE_PATH := "./export-release.html"
NATIVE_EXE_EXPORT_RELEASE_PATH := "./export-release.exe"

_default:
	@just --list

# Build the cartridge in debug mode
build-debug:
	cargo build --target wasm32-unknown-unknown

# Build the cartridge in release mode + strip
build-release:
	@just build-release-nostrip
	@just strip-release

# Build the cartridge in release mode with no strip
build-release-nostrip:
	cargo build --release --target wasm32-unknown-unknown

# Strip release cartridge
strip-release:
	wasm-strip "{{CART_RELEASE_PATH}}"
	wasm-opt -Oz "{{CART_RELEASE_PATH}}" -o "{{CART_RELEASE_PATH}}"

# Build and run WASM (debug)
run-debug-web:
	@just build-debug
	w4 run --no-open --no-qr "{{CART_DEBUG_PATH}}"

# Build the cartridge in release mode + strip and run WASM-4 on web
run-release-web:
	@just build-release
	w4 run --no-open --no-qr "{{CART_RELEASE_PATH}}"

# Build the cartridge in debug mode and run WASM-4 on native mode
run-debug-native:
	@just build-debug
	w4 run-native "{{CART_DEBUG_PATH}}"

# Build the cartridge in release mode + strip and run WASM-4 on native mode
run-release-native:
	@just build-release
	w4 run-native "{{CART_RELEASE_PATH}}"

# Build the cartridge in release mode + strip and export to HTML file
export-release-web:
	@just build-release
	w4 bundle \
		--title "{{CART_TITLE}}" \
		--description "{{CART_DESCRIPTION}}" \
		--html "{{WEB_EXPORT_RELEASE_PATH}}" \
		"{{CART_RELEASE_PATH}}"

# Build the cartridge in release mode + strip and export to Windows EXE file
export-release-exe:
	@just build-release
	w4 bundle \
		--title "{{CART_TITLE}}" \
		--description "{{CART_DESCRIPTION}}" \
		--windows "{{NATIVE_EXE_EXPORT_RELEASE_PATH}}" \
		"{{CART_RELEASE_PATH}}"

# Build and run WASM-4 in watch mode (release, no-strip)
watch:
	CARGO_BUILD_TARGET=wasm32-unknown-unknown w4 watch --no-qr --no-open

# Analyze the debug cartridge
analyze-wasm-debug:
	twiggy top "{{CART_DEBUG_PATH}}"

# Analyze the release + strip cartridge
analyze-wasm-release:
	twiggy top "{{CART_RELEASE_PATH}}"

# Format the code
fmt *ARGS:
	cargo fmt {{ARGS}}

# Run clippy on the code
lint *ARGS:
	cargo clippy --target wasm32-unknown-unknown {{ARGS}}
	cargo clippy --tests {{ARGS}}

# Run tests
test:
	cargo test

# Run CI steps
ci:
	@just fmt "--check"
	@just lint "-- -D warnings"
	@just test
	@just build-release-nostrip

# Build documentation
doc:
	cargo doc

# Clean target folders
clean:
	cargo clean
