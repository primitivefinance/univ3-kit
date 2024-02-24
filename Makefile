build:
	@echo "Building project artifacts."
	forge bind --bindings-path src/bindings --contracts contracts/v3-periphery/ --skip-cargo-toml --module
