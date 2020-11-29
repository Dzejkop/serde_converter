import init, { update, tryUpdateSearchQuery, generateShareQuery } from './pkg/serde_web_converter.js';

const errorMsg = document.querySelector('#error-msg');
const shareLink = document.querySelector('p.share-link');

shareLink.style.display = 'none';

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

   document.querySelector('#left').addEventListener('input', updateOrDisplayError);
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

      document.querySelector('#left').innerText = document.querySelector('#right').innerText;

      updateOrDisplayError();
   });

   document.querySelector('button#share').addEventListener('click', () => {
      shareLink.style.display = 'block';
      const searchQuery = generateShareQuery();
      const wl = window.location;

      shareLink.innerText = `${wl.protocol}//${wl.host}${wl.pathname}?${searchQuery}`;
   });
}

main().then(() => { }).catch((err) => console.error(err));
