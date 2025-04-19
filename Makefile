

run: build
	./target/release/lc3_vm ./samples/2048.obj

build:
	cargo build --release
	@echo "Build complete. Run with ./target/release/lc3_vm ./sample/2048.obj"
clean:
	cargo clean
	@echo "Cleaned build artifacts."