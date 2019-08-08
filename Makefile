all: test fmt

test:
	cargo test

fmt:
	rustfmt src/*.rs src/tests/*.rs benches/*

# `cargo install cargo-travis`
coverage:
	cargo coverage
	open target/kcov/index.html
