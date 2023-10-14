import init, { print, print_with_value, yew_debugger_panel, yew_debugger_collector } from './background/background.js';

var EVENTS_COLLECTOR = []

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
    // console.info(message, sender);
    console.log(sender.tab ?
      "from a content script:" + sender.tab.url :
      "from the extension");

    const api = message["api"] || null;

    console.info(api);

    switch (api) {
      case 'yew-debugger-collector':
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
  const event = message["event"] || null;

  console.log("event")
  console.log(event)
  EVENTS_COLLECTOR.push(event)
  const outcome = yew_debugger_collector(JSON.stringify(event));
  sendResponse(outcome);
}

function handleMessageFromPanel(sender, message, sendResponse) {
  console.log("handleMessageFromPanel");
  console.log(message)
  const senderId = sender["id"] || null;
  const senderOrigin = sender["origin"] || null;


  const urlParts = senderOrigin.split('://');
  const urlScheme = urlParts[0] || null;
  const urlHostname = urlParts[1] || null;

  if (urlScheme === "chrome-extension" && senderId === urlHostname) {
    console.log("Accepted message")
    const command = message["command"] || null;
    const commandName = command["name"] || null;
    if (commandName === "GetEvents") {
      // const outcome = yew_debugger_panel(JSON.stringify(command));
      console.log("EndpointFound: GetEvents")
      const events = JSON.parse(JSON.stringify(EVENTS_COLLECTOR))
      const outcome = {
        isOk: true,
        data: events
      }
      console.log("outcome")
      console.log(outcome)
      sendResponse(outcome);
    } else {
      const outcome = {
        isOk: false,
        error: null
      }
      sendResponse(outcome);
    }
  } else {
    const message_to_send = {
      "message": "Error: Invalid origin for panel.html API"
    };
    console.log(message_to_send)
    sendResponse(message_to_send)
  }


}
