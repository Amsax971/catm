install:
	cargo build
	sudo cp ./target/debug/catm /usr/bin/
	catm GG.gg

uninstall:
	sudo rm -rf /usr/bin/catm