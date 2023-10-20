import init, { welcome_to_background, yew_debugger_collector } from './background/background.js';

var EVENTS_COLLECTOR = []
var CURRENT_TAB = null

chrome.runtime.onInstalled.addListener(() => {
  runDemo();
});

async function runDemo() {
  //* Initialize the WASM module
  await init();

  welcome_to_background();
}

chrome.runtime.onMessage.addListener(
  async function (message, sender, sendResponse) {
    // console.info("background.js - chrome.runtime.onMessage");
    // console.info(message, sender);
    // console.log(sender.tab ?
    //   "from a content script:" + sender.tab.url :
    //   "from the extension");


    const api = message["api"] || null;

    // console.info(api);

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

async function handleMessageFromPanel(sender, message, sendResponse) {
  // console.log("handleMessageFromPanel");
  // console.log(message)
  const senderId = sender["id"] || null;
  const senderOrigin = sender["origin"] || null;


  const urlParts = senderOrigin.split('://');
  const urlScheme = urlParts[0] || null;
  const urlHostname = urlParts[1] || null;

  if (urlScheme === "chrome-extension" && senderId === urlHostname) {
    // console.log("Accepted message")
    const command = message["command"] || null;
    const commandName = command["name"] || null;

    switch (commandName) {
      case "GetEvents":
        const events = JSON.parse(JSON.stringify(EVENTS_COLLECTOR))
        sendResponse({
          isOk: true,
          data: events
        });
        break;

      case "ResetEvents":
        EVENTS_COLLECTOR = []
        sendResponse({
          isOk: true,
          data: EVENTS_COLLECTOR
        });
        break;

      case "ReloadApplication":
        chrome.tabs.query({ active: true, currentWindow: true }, function (arrayOfTabs) {
          chrome.tabs.reload(arrayOfTabs[0].id);
        });
        sendResponse({
          isOk: true,
          data: EVENTS_COLLECTOR
        });
        break;

      default:
        sendResponse({
          isOk: false,
          error: null
        });
    }
  }
}
