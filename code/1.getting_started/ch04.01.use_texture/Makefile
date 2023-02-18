
build-wasm: index.html src/lib.rs
	which wasm-pack || cargo install wasm-pack
	wasm-pack build --target web

clean:
	rm -rf pkg
