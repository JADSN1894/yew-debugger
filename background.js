import init, { print, print_with_value, receive_evelope } from './background/background.js';
// import { ThirtySixChambers } from "yewInterop";

function myJavaScriptFunction(data) {
  console.log("JavaScript function called with data: " + data);
}

chrome.runtime.onInstalled.addListener(() => {
  runDemo();
});

async function runDemo() {
  //* Initialize the WASM module
  await init();

  print();
  print_with_value('');

}


// async function sendMessageToActiveTab(message) {
//   const [tab] = await chrome.tabs.query({ active: true, lastFocusedWindow: true });
//   const response = await chrome.tabs.sendMessage(tab.id, message);
//   // TODO: Do something with the response.

//   console.log("sendMessageToActiveTab");
//   console.log(response);
// }

async function getCurrentTab() {
  let queryOptions = { active: true, lastFocusedWindow: true };
  // `tab` will either be a `tabs.Tab` instance or `undefined`.
  let [tab] = await chrome.tabs.query(queryOptions);
  return tab;
}


// function sendDataToContentScript(data) {
//   chrome.tabs.query({ active: true, currentWindow: true }, function (tabs) {
//     const activeTab = tabs[0];
//     chrome.scripting.executeScript({
//       target: { tabId: activeTab.id },
//       function: chrome.runtime.sendMessage({ data }),
//       args: [data],
//     });
//   });
// }



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

      // const envelope_to_send_string = JSON.stringify(envelope);


      // const dataToSend = { data: "Hello from background.js" };



      //* Call the exported functions from the background WASM module
      const response_from_background_wasm = receive_evelope(message_to_send);
      sendResponse(response_from_background_wasm);
      // document.addEventListener('DOMContentLoaded', () => {
      chrome.scripting.executeScript({
        target: { tabId: sender.tab.id },
        function: function (data) {

          console.log("chrome.scripting.executeScript - data");
          console.log(data);
          // Create and dispatch a custom event in the web page's context
          // var customEvent = new CustomEvent('yewDebuggerMessageCustomEvent', { detail: data });
          // document.dispatchEvent(customEvent);

          // window.postMessage(data, "*");

          // chrome.runtime.sendMessage(data);
        },
        args: [message_to_send] // Pass data as an argument
      });
      // });
      // Define the message you want to send

      // Send the message to the active tab (where your Yew app is running)
      // chrome.runtime.sendMessage(dataToSend);

      // const [tab] = await chrome.tabs.query({ active: true, lastFocusedWindow: true });
      // const responseTabs = await chrome.tabs.sendMessage(tab.id, { greeting: "hello" });

      // console.log("responseTabs - tabs ");
      // console.log(responseTabs);
      // let tabId = await getCurrentTab();
      // chrome.scripting.executeScript({
      //   target: { tabId: tabId, allFrames: true },
      //   files: ['content_scripts/cscript.js'],
      // });

      // chrome.scripting.executeScript({
      //   target: { tabId: sender.tab.id },
      //   function: function (data) {

      //     console.log("chrome.scripting.executeScript - data");
      //     console.log(data);
      //     // Create and dispatch a custom event in the web page's context
      //     var customEvent = new CustomEvent('yewMessage', { detail: data });
      //     document.dispatchEvent(customEvent);
      //   },
      //   args: [message_to_send] // Pass data as an argument
      // });

      // chrome.tabs.executeScript(sender.tab.id, {
      //   code: `
      //       // Create and dispatch a custom event in the web page's context
      //       var customEvent = new CustomEvent('yewCustomEvent', { message: '${response_from_background_wasm}' });
      //       document.dispatchEvent(customEvent);
      //   `
      // });


    }
  }
);

// chrome.runtime.onMessageExternal.addListener((request, sender, sendResponse) => {
//   console.log("Received message from " + sender + ": ", request);
//   sendResponse({ received: true }); //respond however you like
// });

// function onExecuted(result) {
//   console.log(`We made it green`);
// }

// function onError(error) {
//   console.log(`Error: ${error}`);
// }

// const makeItGreen = 'document.body.style.border = "5px solid green"';

// const executing = browser.tabs.executeScript({
//   code: makeItGreen,
// });

// executing.then(onExecuted, onError);

// chrome.tabs.query({ active: true, currentWindow: true }, function (tabs) {
//   const messageToSend = {
//     type: 'SomeMessage',
//     data: 'Hello from background.js!',
//   };


//   const activeTab = tabs[0];

//   // Check if the tab has a Yew component running
//   if (activeTab) {
//     // Send the message to the Yew component's content script
//     chrome.scripting.executeScript({
//       target: { tabId: activeTab.id },
//       function: (message) => {
//         // Dispatch a custom event to notify the Yew component
//         const event = new CustomEvent('backgroundMessage', { detail: message });
//         window.dispatchEvent(event);
//       },
//       args: [messageToSend],
//     });
//   }
// });

// chrome.tabs.onUpdated.addListener(async function (tabId, changeInfo, tab) {
//   if (changeInfo && changeInfo.status == "complete") {
//     console.log("IT WORKS");
//     const messageToSend = {
//       type: 'SomeMessage',
//       data: 'Hello from background.js!',
//     };

//     // chrome.scripting.executeScript({
//     //   target: { tabId: tabId },
//     //   function: (message) => {
//     //     // Dispatch a custom event to notify the Yew component
//     //     const event = new CustomEvent('backgroundMessage', { detail: message });
//     //     window.dispatchEvent(event);
//     //   },
//     //   args: [messageToSend],
//     // });

//     if (tab.url.includes('chrome://')) {
//       console.log('can`t run on start page')
//     } else {

//       chrome.scripting.executeScript({
//         target: { tabId: tabId },
//         function: (message) => {
//           console.log("message");
//           console.log(message);

//           const event = new CustomEvent('backgroundMessage', { detail: message });
//           window.dispatchEvent(event);
//         },
//         args: [messageToSend],
//       });
//     }

//   }



// const activeTab = tabs[0];
// Check if the tab has a Yew component running
// Send the message to the Yew component's content script
// chrome.scripting.executeScript({
//   target: { tabId: tabId },
//   function: (message) => {
//     // Dispatch a custom event to notify the Yew component
//     const event = new CustomEvent('backgroundMessage', { detail: message });
//     window.dispatchEvent(event);
//   },
//   args: [messageToSend],
// });

//code in here will run every time a user goes onto a new tab, so you can insert your scripts into every new tab
// });

// chrome.tabs.onActivated.addListener((activeInfo) => {
//   // Determine the active tab's ID
//   const tabId = activeInfo.tabId;

//   // Create a message to send
//   const messageToSend = {
//     type: 'SomeMessage',
//     data: 'Hello from background.js!',
//   };

//   // Inject a content script into the active tab
//   chrome.scripting.executeScript({
//     target: { tabId: tabId },
//     function: (message) => {
//       // Dispatch a custom event to notify the Yew component
//       const event = new CustomEvent('backgroundMessage', { detail: message });
//       window.dispatchEvent(event);
//     },
//     args: [messageToSend],
//   });
// });