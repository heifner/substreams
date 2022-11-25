SUBSTREAM := *

.PHONY: build
build:
	cargo build --manifest-path=./substreams/$(SUBSTREAM)/Cargo.toml --target wasm32-unknown-unknown --release

.PHONY: codegen
codegen:
	substreams protogen ./substreams/$(SUBSTREAM)/substreams.yaml --exclude-paths="sf/substreams,google"
