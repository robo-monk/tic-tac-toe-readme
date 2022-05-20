addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

const { handle } = wasm_bindgen;
const instance = wasm_bindgen(wasm);

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  await instance;
//   let logger = 3;
    function logger(params) {
        console.log('>>', params);
    }

  return await handle(TICTAC, request, logger);
}
