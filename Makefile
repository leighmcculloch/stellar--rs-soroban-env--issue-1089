.PHONY: default all test build deploy fmt clean

include .env

default: build

test:
	cargo test

build:
	soroban contract build --out-dir out --package counter
	soroban contract build --out-dir out --package deployer
	soroban contract build --out-dir out --package installer
	@ls -l out/*.wasm

deploy:
	SOROBAN_RPC_URL=${SOROBAN_RPC_URL} SOROBAN_NETWORK_PASSPHRASE=${SOROBAN_NETWORK_PASSPHRASE} soroban config identity fund
	SOROBAN_RPC_URL=${SOROBAN_RPC_URL} SOROBAN_NETWORK_PASSPHRASE=${SOROBAN_NETWORK_PASSPHRASE} soroban contract deploy --wasm out/installer.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
	rm -fr out
