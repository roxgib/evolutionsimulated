wasm-pack build --target web
rmdir -f web/pkg
cp -r pkg web
cd web
npx serve