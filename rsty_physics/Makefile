ONESHELL:


#The following recipes assume that GODOT4_BIN=~/Applications/Godot_v4.1.1-stable_linux.x86_64 is set
build:
	cargo build
build_cross_windows:
	cross build --target x86_64-pc-windows-gnu

build_wasm:
	cargo +nightly build  --target wasm32-unknown-emscripten -Zbuild-std


build_all:
	make build
	make build_cross_windows
	make build_wasm


#Assumes that Gunner is cloned at /home/lgomez/Gunner
update_gunner_modules: build_all
	rm -f /home/lgomez/Gunner/Gunner/bin/*
	cp target/debug/librsty_physics.so /home/lgomez/Gunner/Gunner/bin
	cp target/x86_64-pc-windows-gnu/debug/rsty_physics.dll /home/lgomez/Gunner/Gunner/bin
	cp target/wasm32-unknown-emscripten/debug/rsty_physics.wasm /home/lgomez/Gunner/Gunner/bin

clean:
	rm Cargo.lock && cargo clean
