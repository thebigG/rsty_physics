ONESHELL:

build:
	GODOT4_BIN=~/Applications/Godot_v4.1.1-stable_linux.x86_64 cargo build
build_cross_windows:
	cross build --target x86_64-pc-windows-gnu

build_wasm:
	cargo +nightly build  --target wasm32-unknown-emscripten -Zbuild-std


clean:
	rm Cargo.lock && cargo clean
