install:
	cargo build --release
	mv target/release/stringsolver /usr/local/bin/stringsolver