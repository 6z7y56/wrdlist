ifeq ($(shell id -u),0)
$(error Don't run make with sudo. Just use 'make install' instead)
endif

install:
	cargo build --release
	sudo cp target/release/wrdlist /usr/bin

uninstall:
	sudo rm /usr/bin/wrdlist

clean: 
	cargo clean

