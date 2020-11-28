import init, { update, tryUpdateSearchQuery, generateShareQuery } from './pkg/serde_web_converter.js';

const errorMsg = document.querySelector('#error-msg');

function updateError(error) {
   errorMsg.textContent = '';
   if (!error) {
      return;
   }

   if (error === null) {
      return;
   }

   console.error(error);
   errorMsg.textContent = error.toString();
}

function updateOrDisplayError() {
   try {
      update();
      updateError(null);
   } catch (error) {
      updateError(error);
   }
}

async function main() {
   await init('/serde_web_converter_bg.wasm');

   try {
      tryUpdateSearchQuery();
      updateError(null);
   } catch (error) {
      updateError(error);
   }

   document.querySelector('textarea#left').addEventListener('input', updateOrDisplayError);
   document.querySelector('select#input-format').addEventListener('input', updateOrDisplayError);
   document.querySelector('select#target-format').addEventListener('input', updateOrDisplayError);
   document.querySelector('#csv-options label input#has-header').addEventListener('input', updateOrDisplayError);
   document.querySelector('#share').addEventListener('click', () => {
      console.log(generateShareQuery());
   });

   document.querySelector('button#flip').addEventListener('click', () => {
      let inputFormat = document.querySelector('select#input-format');
      let outputFormat = document.querySelector('select#target-format');

      let tmp = inputFormat.value;
      inputFormat.value = outputFormat.value;
      outputFormat.value = tmp;

      document.querySelector('#left').value = document.querySelector('#right').value;

      updateOrDisplayError();
   });
}


main();
