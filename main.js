import init from './pkg/serde_web_converter.js';
async function main() {
   await init('/pkg/serde_web_converter_bg.wasm');
}
main()