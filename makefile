all: wasm dev message

wasm:
	cd src/ffi/wasm && wasm-pack build --target web

dev:
	cd dev && rm -rf node_modules && rm -rf package-lock.json && npm cache clean --force && npm i && npm run serve

message:
	@echo "All done! Enjoy your development."

.PHONY: wasm dev message
