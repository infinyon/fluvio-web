WASM_TARGET = wasm32-unknown-unknown

fmt:
	cargo fmt -- --check

WS_PROXY = fluvio-ws-proxy
WEB = fluvio-web
COUNTER = fluvio-counter

clippy:
	cargo clippy --all-features --tests -p $(WS_PROXY) -- -D warnings
	cargo clippy --all-features --tests --target $(WASM_TARGET) -p $(WEB) -- -D warnings
	cargo clippy --all-features --tests --target $(WASM_TARGET) -p $(COUNTER) -- -D warnings