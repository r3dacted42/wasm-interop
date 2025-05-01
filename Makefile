# ---- CONFIGURATION ----
FROM_LANG := rust
TO_LANG := cpp
INPUT_DIR := rustmath-demo
INPUT_SRC := $(INPUT_DIR)/src/lib.rs
MODULE_NAME := rustmath
OUT_DIR := $(MODULE_NAME)-bindings
BIN_NAME := wasm-interop
BIN_PATH := target/release/$(BIN_NAME)
WASM_TARGET := wasm32-unknown-unknown
WASM_PATH := $(INPUT_DIR)/target/$(WASM_TARGET)/release/*.wasm
WASM_OUT := $(OUT_DIR)/$(MODULE_NAME).wasm
BINDINGS_STAMP := $(OUT_DIR)/bindings.stamp
WASM_API_ARCHIVE := wasm-api.tar.gz
WASM_API_DIR := $(OUT_DIR)/wasm-api

# ---- MAIN TARGET ----
all: $(BIN_PATH) $(BINDINGS_STAMP) $(WASM_OUT)

# ---- Build binary tool ----
$(BIN_PATH):
	cargo build --release

# ---- Generate bindings using your CLI and optionally extract wasm-api ----
$(BINDINGS_STAMP): $(BIN_PATH) $(INPUT_SRC)
	mkdir -p $(OUT_DIR)
	$(BIN_PATH) --from=$(FROM_LANG) --to=$(TO_LANG) --input=$(INPUT_SRC) --module=$(MODULE_NAME)
	@if [ "$(TO_LANG)" = "cpp" ]; then \
		echo "Extracting wasm-api for C++..."; \
		tar -xzf $(WASM_API_ARCHIVE) -C $(OUT_DIR); \
	fi
	touch $@

# ---- Compile Rust to WASM ----
$(WASM_OUT): $(INPUT_SRC)
	cd $(INPUT_DIR) && cargo build --release --target $(WASM_TARGET)
	cp $(WASM_PATH) $(WASM_OUT)

# ---- Cleanup ----
clean:
	rm -rf $(OUT_DIR) $(BIN_PATH) $(WASM_PATH)

.PHONY: all clean
