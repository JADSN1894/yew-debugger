// Create a web worker
const worker = new Worker('your_worker.js');

// Add a listener to handle messages from the worker
worker.addEventListener('message', (event) => {
    console.warn('Message from worker:', event.data);

    // Respond to the message from Rust
    // (You can add your response logic here)
});
