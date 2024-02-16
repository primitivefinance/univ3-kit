build:
	@echo "Building project artifacts."
	forge bind --bindings-path src/bindings --contracts contracts/ --skip-cargo-toml --module
