build:
	cargo build --release

install: build
	install ./target/release/exec ~/.config/sway/

debug-build:
	cargo build

test: debug-build
	./target/debug/exec alacritty alacritty
