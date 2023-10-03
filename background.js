import init, { print, print_with_value, receive_evelope } from './background/background.js';

chrome.runtime.onInstalled.addListener(() => {
  runDemo();
});

async function runDemo() {
  //* Initialize the WASM module
  await init();

  print();
  print_with_value('background.js');
}

chrome.runtime.onMessage.addListener(
  function (message, sender, sendResponse) {
    console.info(message, sender);
    console.log(sender.tab ?
      "from a content script:" + sender.tab.url :
      "from the extension");

    if (message["recipient"] === "yew-debugger") {
      const envelope = message["envelope"] || null;

      const message_to_send = {
        "message": JSON.stringify(envelope)
      };

      // const envelope_to_send_string = JSON.stringify(envelope);

      //* Call the exported functions from the background WASM module
      const response_from_background_wasm = receive_evelope(message_to_send);
      sendResponse(response_from_background_wasm);
    }

  }
);