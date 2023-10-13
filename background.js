import init, { print, print_with_value, receive_evelope } from './background/background.js';

chrome.runtime.onInstalled.addListener(() => {
  runDemo();
});

async function runDemo() {
  //* Initialize the WASM module
  await init();

  print();
  print_with_value('');

}

chrome.runtime.onMessage.addListener(
  async function (message, sender, sendResponse) {
    console.info("background.js - chrome.runtime.onMessage");
    console.info(message, sender);
    console.log(sender.tab ?
      "from a content script:" + sender.tab.url :
      "from the extension");

    if (message["recipient"] === "yew-debugger") {
      const envelope = message["envelope"] || null;

      const message_to_send = {
        "message": JSON.stringify(envelope)
      };

      // chrome.storage.local.get({ events: [] }, function (result) {
      //   // the input argument is ALWAYS an object containing the queried keys
      //   // so we select the key we need
      //   var events = result.events;
      //   events.push(JSON.stringify(envelope));
      //   // set the new array value to the same key
      //   chrome.storage.local.set({ events: events }, function () {
      //     // you can use strings instead of objects
      //     // if you don't  want to define default values
      //     chrome.storage.local.get('events', function (result) {
      //       console.log(result.events)
      //     });
      //   });
      // });


      //* Call the exported functions from the background WASM module
      const response_from_background_wasm = receive_evelope(message_to_send);
      sendResponse(response_from_background_wasm);
    }
  }
);