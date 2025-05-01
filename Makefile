# Makefile for Rust to C++ WASM binding generation

# ---- CONFIGURATION ----
INPUT_DIR := rustmath-demo
INPUT_SRC := ./$(INPUT_DIR)/src/lib.rs
WASM_TARGET := wasm32-unknown-unknown
MODULE_NAME := rustmath
OUT_DIR := $(MODULE_NAME)-bindings
BIN_NAME := wasm-interop
BIN_PATH := target/release/$(BIN_NAME)
WASM_OUT := $(OUT_DIR)/$(MODULE_NAME).wasm
CPP_FILE := $(OUT_DIR)/$(MODULE_NAME).cpp
H_FILE := $(OUT_DIR)/$(MODULE_NAME).h
MF_FILE := $(OUT_DIR)/Makefile
BINDINGS_STAMP := $(OUT_DIR)/bindings.stamp
WASM_API_ARCHIVE := wasm-api.tar.gz
WASM_API_DIR := $(OUT_DIR)/wasm-api

all: $(BIN_PATH) $(CPP_FILE) $(H_FILE) $(MF_FILE) $(WASM_OUT) $(JS_OUT) $(WASM_API_DIR)

$(BIN_PATH):
	cargo build --release

$(WASM_API_DIR):
	mkdir -p $(OUT_DIR)
	tar -xzf $(WASM_API_ARCHIVE) -C $(OUT_DIR)

$(BINDINGS_STAMP): $(BIN_PATH) $(INPUT_SRC)
	mkdir -p $(OUT_DIR)
	$(BIN_PATH) $(INPUT_SRC) $(MODULE_NAME)
	touch $@

$(CPP_FILE) $(H_FILE) $(MF_FILE): $(BINDINGS_STAMP)

$(WASM_OUT) $(JS_OUT): $(INPUT_SRC)
	cd $(INPUT_DIR) && cargo build --release --target $(WASM_TARGET)
	cp $(INPUT_DIR)/target/$(WASM_TARGET)/release/*.wasm $(WASM_OUT)

clean:
	rm -rf $(OUT_DIR) ./target $(INPUT_DIR)/target $(INPUT_DIR)/target

.PHONY: all clean
