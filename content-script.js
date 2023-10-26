console.log("content-script.js loaded!");

window.addEventListener(
    "message",
    async (event) => {
        const msg = event["data"] || null;
        const response = await chrome.runtime.sendMessage(msg);

        // console.log("response");
        // console.log(response);
    },
    false,
);
