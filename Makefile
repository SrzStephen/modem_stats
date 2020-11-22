run:
	cargo run

release:
	CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld REALGCC=arm-linux-gnueabihf-gcc-8 TARGET_CC=musl-gcc CFLAGS="-mfloat-abi=softfp" cross build --release --target armv7-unknown-linux-musleabi

release_travis:
	CFLAGS_armv7-unknown-linux-musleabi=arm-linux-gnueabihf-gcc-5 CC_armv7_unknown_linux_musleabi=arm-linux-gnueabihf-gcc-5 CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld REALGCC=arm-linux-gnueabihf-gcc-5 TARGET_CC=musl-gcc CFLAGS="-mfloat-abi=softfp" cross build --release --target armv7-unknown-linux-musleabi

bump:
	bumpversion patch

prep-test:
	pip3 install ubus_simulator/.

test:
	cargo test
	
install:
	cargo install --path .
