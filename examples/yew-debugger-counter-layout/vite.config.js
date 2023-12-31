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
    },
})