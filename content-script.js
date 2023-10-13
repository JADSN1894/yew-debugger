async function sendMessageToActiveTab(message) {
    const [tab] = await chrome.tabs.query({ active: true, lastFocusedWindow: true });
    const response = await chrome.tabs.sendMessage(tab.id, message);
    // TODO: Do something with the response.

    console.log("sendMessageToActiveTab");
    console.log(response);
}

window.addEventListener(
    "message",
    async (event) => {
        console.log(event);
        const msg = event["data"] || null;

        console.log("content-script");
        console.log("msg");
        console.log(msg);

        const response = await chrome.runtime.sendMessage(msg);

        console.log("response");
        console.log(response);

        const from_background_wasm = JSON.parse(response["message"])
        console.log("from_background_wasm");
        console.log(from_background_wasm);


        // const event_to_panel = new CustomEvent('yewMessage', {
        //     detail: {
        //         message: from_background_wasm,
        //     }
        // });
        // document.dispatchEvent(event_to_panel);

        // chrome.tabs.executeScript(sender.tab.id, {
        //     code: `
        //         // Create and dispatch a custom event in the web page's context
        //         console.log("chrome.tabs.executeScript")
        //         var customEvent = new CustomEvent('yewCustomEvent', { message: '${response_from_background_wasm}' });
        //         document.dispatchEvent(customEvent);
        //     `
        // });
    },
    false,
);

// chrome.runtime.onMessageExternal.addListener(
//     function (request, sender, sendResponse) {

//         console.info(request);
//         console.info(sender);
//         console.info(sendResponse);
//     });


// // Listen for connections from DevTools panels
// chrome.runtime.onConnect.addListener(function (port) {
//     // Store the connection object
//     devToolsConnection = port;

//     // Set up a listener for messages from the DevTools panel
//     devToolsConnection.onMessage.addListener(function (message) {
//         // Handle messages from the DevTools panel here
//         console.log("Received message from DevTools panel:", message);

//         // Send a response back to the DevTools panel if needed
//         // devToolsConnection.postMessage({ response: "Data from background script" });
//     });

//     // Optionally, handle disconnect events
//     devToolsConnection.onDisconnect.addListener(function () {
//         devToolsConnection = null;
//         console.log("DevTools panel disconnected");
//     });
// });