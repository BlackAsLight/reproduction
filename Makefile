build:
	LEPTOS_OUTPUT_NAME=app cargo build
	cargo build --no-default-features --target=wasm32-unknown-unknown --features=client --profile dev-wasm
	mkdir -p static/wasm/
	wasm-bindgen --out-dir static/wasm/ --out-name app --target web --no-typescript target/wasm32-unknown-unknown/dev-wasm/reproduction.wasm

run:
	make build
	target/debug/reproduction
