init:
	maturin init

install:
	uv pip install -r requirements.txt

build:
	cargo build --release
	cp target/release/libpycalc_cli.so .

clean:
	rm -f libpycalc_cli.so