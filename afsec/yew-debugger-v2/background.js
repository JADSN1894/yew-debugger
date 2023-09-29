import init, { print, print_with_value } from './background/pkg/helloworld_demo.js';

chrome.runtime.onInstalled.addListener(() => {
  runDemo();
});

async function runDemo() {
  // Initialize the WASM module
  await init();

  // Call the exported functions from the WASM module
  print();
  print_with_value('John');
}