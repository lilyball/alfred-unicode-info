all: ../unicode
.PHONY: all

../unicode: target/release/unicode
	cp -f $^ $@

target/release/unicode:
	cargo build --release
.PHONY: target/release/unicode

clean:
	cargo clean
.PHONY: clean
