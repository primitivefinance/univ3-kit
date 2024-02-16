build:
	@echo "Building project artifacts."
	forge bind --bindings-path src/bindings --contracts contracts/v3-core/contracts --skip-cargo-toml --module
