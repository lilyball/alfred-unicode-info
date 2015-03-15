all: target/release/unicode
.PHONY: all

target/release/unicode:
	cargo build --release
.PHONY: target/release/unicode

install: target/release/unicode
	./alfred-install-workflow/install-workflow.sh target/release/unicode
.PHONY: install

clean:
	cargo clean
.PHONY: clean
