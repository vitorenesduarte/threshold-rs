all: test fmt

test:
	cargo test

fmt:
	rustfmt src/*.rs src/tests/*.rs benches/*

publish: test
	cargo doc
	cargo package
	cargo publish

# `cargo install cargo-travis`
coverage:
	cargo coverage
	open target/kcov/index.html
