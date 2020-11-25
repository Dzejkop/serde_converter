import init, { CsvOptions } from './serde_web_converter.js';

async function main() {
   await init('/serde_web_converter_bg.wasm');

   let x = new CsvOptions(true);
   console.log(x.has_header);
}


main();
