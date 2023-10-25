console.log("content-script.js loaded!");

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
    },
    false,
);
