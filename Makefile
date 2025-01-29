WASM_TARGET = wasm32-unknown-unknown
RELEASE = v0.2.0

fmt:
	cargo fmt -- --check

WS_PROXY = fluvio-ws-proxy
WEB = fluvio-web
COUNTER = fluvio-counter

clippy:
	cargo clippy --all-features --tests -p $(WS_PROXY) -- -D warnings
	cargo clippy --all-features --tests --target $(WASM_TARGET) -p $(WEB) -- -D warnings
	cargo clippy --all-features --tests --target $(WASM_TARGET) -p $(COUNTER) -- -D warnings


release:
	gh release create  $(RELEASE)