SHELL := /bin/bash
.PHONY: help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

recreate_local_containers: ## Remove and recreate all containers (including the database)
	./local/recreate_containers.sh

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Format the project using cargo
	@rustup component add rustfmt
	cargo fmt

bump: ## Bump the version of the project
	@cargo bump patch

docs:
	cargo doc --open
