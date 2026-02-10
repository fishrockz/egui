rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release --lib
cp ../../target/wasm32-unknown-unknown/release/custom_wgpu.wasm .


echo "ensuring basic-http-server is installed…"
cargo install basic-http-server
# cargo isntall wasm-bindgen-cli
cargo install --force --locked wasm-bindgen-cli --version 0.2.100 
wasm-bindgen ../../target/wasm32-unknown-unknown/release/custom_wgpu.wasm --out-dir web_demo --no-modules --no-typescript
cp index.html web_demo

echo "starting server…"
echo "serving at http://localhost:8888
basic-http-server --addr 0.0.0.0:8888 web_demo/
