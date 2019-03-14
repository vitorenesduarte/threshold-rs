fmt:
	rustfmt src/* tests/* benches/*

# `cargo install cargo-travis`
coverage:
	cargo coverage
	open target/kcov/index.html
