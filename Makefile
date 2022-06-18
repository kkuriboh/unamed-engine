all: bundle clean
	mv pkg web/ && cp -r web/pkg web/dist

cargo:
	cargo clippy && cargo fmt

bundle:
	wasm-pack build --target web

clean:
	rm -fr web/pkg web/dist/pkg
