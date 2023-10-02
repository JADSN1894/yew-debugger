self.addEventListener('message', (event) => {
    const recipient = event.data["recipient"] || null;
    console.log(recipient);
    if (recipient == "yew-debugger") {
        const messageFromRust = JSON.stringify(event.data);
        const responseMessage = `Received message from Rust: ${messageFromRust}`;
        console.warn(responseMessage);
        
    } else {
        const messageFromRust = JSON.stringify(event.data);
        const responseMessage = `Not mine!: ${messageFromRust}`;
        console.log(responseMessage);
    }
    
});
