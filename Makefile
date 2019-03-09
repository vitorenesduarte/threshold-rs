fmt:
	rustfmt tests/* benches/* src/*

coverage:
	cargo coverage
	open target/kcov/index.html
