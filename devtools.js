chrome.devtools.panels.create('Yew Debugger', 'icon.png', 'panel.html', panel => {

  console.log('Yew Debugger devtools tab clicked');
  // chrome.devtools.inspectedWindow.eval("Waat");

  panel.onShown.addListener((extPanelWindow) => {

    console.log("extPanelWindow");
    console.log(extPanelWindow);
    console.log('devtools - panel.onShown');

    // Function to handle the connection to the DevTools panel
  });
});

// chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
//   // Messages from content scripts should have sender.tab set
//   console.log('devtools - chrome.runtime.onMessage');

// });

// // Create a connection to the background service worker
// const backgroundPageConnection = chrome.runtime.connect({
//   name: "devtools-page"
// });

// // Relay the tab ID to the background service worker
// backgroundPageConnection.postMessage({
//   name: 'init',
//   tabId: chrome.devtools.inspectedWindow.tabId
// });