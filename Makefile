build:
	gcc src/app.c -Oz -o src/app.o
	cargo run --bin make_signature
	cargo build --bin memfd-ed25519

clean:
	rm -rf src/*.o src/.main.rs

run:
	target/*/memfd-ed25519 &

tamper:
	perl -pi -e 's/verified/ver1fied/' target/*/memfd-ed25519

.PHONY: all
