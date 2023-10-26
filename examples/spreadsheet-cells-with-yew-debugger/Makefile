.PHONY: clean

clean:
	cargo clean
	rm -rfv frontend-wasm
	rm -rfv dist

debug:
	@wasm-pack build --target web --no-typescript --mode force --dev --out-name index --out-dir ../frontend-wasm ./frontend

release:
	@wasm-pack build --target web --no-typescript --mode force --release --out-name index --out-dir ../frontend-wasm ./frontend

tailwind-debug:
	./tailwindcss --input=input.css --output=output.css

tailwind-release:
	./tailwindcss --input=input.css --output=output.css --minify

distrib:
	@mkdir -pv dist
	@cp -rfv ./frontend-wasm dist
	@cp -fv ./index.html dist
	@cp -fv ./output.css dist
	@cp -fv ./index.js dist

build-debug: clean tailwind-debug debug distrib

build-release: clean tailwind-release release distrib

python-server:
	python3 -m http.server --directory dist