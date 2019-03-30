PREFIX?=/usr/local
DYLIB_EXT?=$(shell if [ `uname` == "Darwin" ]; then echo "dylib"; else echo "so"; fi)


all: library

library:
	cargo build --manifest-path ../Cargo.toml --features="c_api" --release

install: library
	install -d $(PREFIX)/lib/
	install -m 644 ../target/release/libremote_pid.a $(PREFIX)/lib/
	install -m 644 ../target/release/libremote_pid.$(DYLIB_EXT) $(PREFIX)/lib/
	install -d $(PREFIX)/include/
	install -m 644 remote_pid.h $(PREFIX)/include/

.PHONY: uninstall
uninstall:
	rm $(PREFIX)/lib/libremote_pid.a
	rm $(PREFIX)/lib/libremote_pid.$(DYLIB_EXT)
	rm $(PREFIX)/include/remote_pid.h

.PHONY: clean
clean:
	rm -rf ../target