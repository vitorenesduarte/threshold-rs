all: test fmt

test:
	cargo test

fmt:
	rustup override set nightly
	rustfmt src/*.rs src/tests/*.rs benches/*
	rustup override set stable

publish: test
	cargo doc
	cargo package
	cargo publish

# `cargo install cargo-travis`
coverage:
	cargo coverage
	open target/kcov/index.html
