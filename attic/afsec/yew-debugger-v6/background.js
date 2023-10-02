import init, { print, print_with_value } from './background/background.js';

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

var YEW_DEBUGGER_MESSAGES = [];


chrome.runtime.onMessage.addListener(
  function (message, sender, sendResponse) {
    console.warn(message, sender);
    console.log(sender.tab ?
      "from a content script:" + sender.tab.url :
      "from the extension");


    if (message["recipient"] === "yew-debugger") {
      // if (message.message === "PING?") {
      //   YEW_DEBUGGER_MESSAGES.push(message.message);
      //   console.warn(YEW_DEBUGGER_MESSAGES);

      // }
      const envelope = message["envelope"] || null;
      YEW_DEBUGGER_MESSAGES.push(envelope);

      console.warn(YEW_DEBUGGER_MESSAGES)
      sendResponse({ message: "PONG!" });
    }

  }
);