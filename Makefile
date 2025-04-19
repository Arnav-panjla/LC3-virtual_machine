

run_2048: build
	./target/release/lc3_vm ./samples/2048.obj

build: clean
	cargo build --release
	@echo "Build complete. Run with ./target/release/lc3_vm ./sample/2048.obj"
clean:
	cargo clean
	@echo "Cleaned build artifacts."