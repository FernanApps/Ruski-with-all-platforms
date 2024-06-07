// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {hello_world, encrypt, decrypt} from "./pkg/library_wasm.js";

async function initializeWasm() {
  await wasmInit();
  window.hello_world = hello_world;
  window.encrypt = encrypt;
  window.decrypt = decrypt;

  // Set the initial value of inputText after wasm initialization
  document.getElementById('inputText').value = hello_world();
}

await initializeWasm();
