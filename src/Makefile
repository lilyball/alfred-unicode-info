.PHONY: all clean

all: ../unicode

../unicode:
	rustc -O --out-dir .. --dep-info pkg.d pkg.rs

include pkg.d

clean:
	-rm -f ../unicode
