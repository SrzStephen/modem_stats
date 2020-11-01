run:
	cargo run

release:
	CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld REALGCC=arm-linux-gnueabihf-gcc-8 TARGET_CC=musl-gcc CFLAGS="-mfloat-abi=softfp" cross build --release --target armv7-unknown-linux-musleabi

bump:
	bumpversion patch