import { defineConfig } from 'vite'

export default defineConfig({
    esbuild: {
        supported: {
            'top-level-await': true //browsers can handle top-level-await features
        },
    },
    server: {
        port: 8000,
        host: "127.0.0.1",

        // proxy: {
        //     '/wasm/': {
        //         target: 'http://localhost:3000', // Adjust the URL to match your Yew development server
        //         changeOrigin: true,
        //         rewrite: (path) => path.replace(/^\/wasm\//, '/'),
        //     },
        // },
        // proxy: {
        //     '/wasm/': 'http://localhost:3000/', // Adjust the URL to match your wasm-pack setup
        // },
    },
})