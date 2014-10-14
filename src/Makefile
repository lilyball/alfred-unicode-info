.PHONY: all clean

all: ../unicode

../unicode: target/release/unicode
	cp -f $^ $@

target/release/unicode:
	cargo build --release

clean:
	cargo clean
