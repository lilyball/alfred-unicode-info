all: target/release/unicode
.PHONY: all

target/release/unicode:
	cargo build --release
.PHONY: target/release/unicode

install: target/release/unicode
	./alfred-install-workflow/install-workflow.sh target/release/unicode

clean:
	cargo clean
	-rm -f version
.PHONY: clean

ifeq ($(OUT_DIR),)
cargo-prep:
	$(error make cargo-prep must be called by cargo)
.PHONY: cargo-prep
else
# the rest of the Makefile is only visible to Cargo

cargo-prep: $(OUT_DIR)/version
.PHONY: cargo-prep

$(OUT_DIR)/version:
	printf "%s (%s)\n" "$$(git describe --always --dirty)" "$$(date +'%F %T %z')" > $@
.PHONY: $(OUT_DIR)/version

endif
