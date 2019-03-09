fmt:
	rustfmt src/* tests/* benches/*

coverage:
	cargo coverage
	open target/kcov/index.html
