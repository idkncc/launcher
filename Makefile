CARGO := cargo

.PHONY: install-dev install

install-dev: target/debug/liblauncher.so
	cp target/debug/liblauncher.so ~/.config/waybar/custom

install: target/debug/liblauncher.so
	cp target/release/liblauncher.so ~/.config/waybar/custom

target/debug/liblauncher.so: src/*
	$(CARGO) build

target/release/liblauncher.so: src/*
	$(CARGO) build --release
