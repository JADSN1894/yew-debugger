import init, { print, print_with_value, receive_envelope } from './background/background.js';

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

    const recipient = message["recipient"] || null;

    console.info(recipient);

    switch (recipient) {
      case 'yew-debugger':
        handleMessageFromContentScript(sender, message, sendResponse)
        break
      case 'yew-debugger-panel':
        handleMessageFromPanel(sender, message, sendResponse)
        break
      default:
        break
    }
  }
);

function handleMessageFromContentScript(sender, message, sendResponse) {
  console.log("handleMessageFromContentScript");
  console.log(message)
  const envelope = message["envelope"] || null;

  const message_to_send = {
    "message": JSON.stringify(envelope)
  };

  //* Call the exported functions from the background WASM module
  const response_from_background_wasm = receive_envelope(JSON.stringify(message_to_send));
  const outcome = JSON.parse(response_from_background_wasm)
  console.log(outcome)
  sendResponse(outcome);
}

function handleMessageFromPanel(sender, message, sendResponse) {
  console.log("handleMessageFromPanel");
  console.log(message)
  const senderId = sender["id"] || null;
  const senderOrigin = sender["origin"] || null;
  console.log(senderId, senderOrigin)

  const urlParts = senderOrigin.split('://');
  const urlScheme = urlParts[0] || null;
  const urlHostname = urlParts[1] || null;
  // console.log(urlScheme, urlHostname)
  if (urlScheme === "chrome-extension" && senderId === urlHostname) {
    console.log("Accepted message")
    const envelope = message["envelope"] || null;

    const message_to_send = {
      "message": envelope
    };

    //* Call the exported functions from the background WASM module
    const response_from_background_wasm = receive_envelope(JSON.stringify(message_to_send));
    const outcome = JSON.parse(response_from_background_wasm)
    console.log(outcome)
    sendResponse(outcome);
  } else {
    const message_to_send = {
      "message": "Error: Invalid origin for panel.html API"
    };
    console.log(message_to_send)
    sendResponse(message_to_send)
  }


}
