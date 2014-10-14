all: ../unicode
.PHONY: all

../unicode: target/release/unicode
	cp -f $^ $@

target/release/unicode:
	cargo build --release
.PHONY: target/release/unicode

clean:
	cargo clean
	-rm -f version
.PHONY: clean

version:
	printf "%s (%s)\n" "$$(git describe --always --dirty)" "$$(date +'%F %T %z')" > $@
.PHONY: version
