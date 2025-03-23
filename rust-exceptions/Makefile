init:
	maturin init

build:
	PYTHON_LIBRARY="$(shell python3.12-config --prefix)/lib/libpython3.12.dylib" \
	RUSTFLAGS="-C link-args=-Wl,-rpath,$(shell python3.12-config --prefix)/lib" \
	cargo build --release
	cp target/release/librust_exceptions.dylib .
	ln -sf librust_exceptions.dylib librust_exceptions.so

clean:
	cargo clean
	rm -f librust_exceptions.dylib librust_exceptions.so