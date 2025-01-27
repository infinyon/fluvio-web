BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/common.mk

PROJ_NAME = fluvio-web
build:
	cargo build -p $(PROJ_NAME) $(WASM_BUILD_SETTING)


# init directory to satify clippy
fake_dist:
	mkdir -p release/dist

clean:
	rm -rf release/dist