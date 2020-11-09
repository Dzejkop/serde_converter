import init from './serde_web_converter.js';

async function main() {
   await init('/serde_web_converter_bg.wasm');
}

main();