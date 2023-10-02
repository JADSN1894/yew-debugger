// setTimeout(async () => {
//     const msg = { recipient: "YewDebugger", message: "PING?" };
//     const response = await chrome.runtime.sendMessage(msg);
//     // do something with response here, not outside the function
//     console.warn(response);
// }, 1000)

window.addEventListener(
    "message",
    async (event) => {
        console.warn(event);
        // const msg = { recipient: "YewDebugger", message: "PING?" };
        const msg = event["data"] || null;
        const response = await chrome.runtime.sendMessage(msg);
        console.log(response);
    },
    false,
);