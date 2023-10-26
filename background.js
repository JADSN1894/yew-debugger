import init, { welcome_to_background, yew_debugger_collector } from './background/background.js';

var EVENTS_COLLECTOR = []

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
    const api = message["api"] || null;

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
  const event = message["event"] || null;

  EVENTS_COLLECTOR.push(event)

  const outcome = yew_debugger_collector(JSON.stringify(event));
  sendResponse(outcome);
}

async function handleMessageFromPanel(sender, message, sendResponse) {
  const senderId = sender["id"] || null;
  const senderOrigin = sender["origin"] || null;

  const urlParts = senderOrigin.split('://');
  const urlScheme = urlParts[0] || null;
  const urlHostname = urlParts[1] || null;

  if (urlScheme === "chrome-extension" && senderId === urlHostname) {
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
        chrome.windows.getCurrent(currentWindow => {
          chrome.tabs.query({ active: true, windowId: currentWindow.id }, arrayOfTabs => {
            const tabId = arrayOfTabs[0].id;
            chrome.tabs.reload(tabId, { bypassCache: true });
          });
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
