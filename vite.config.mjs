import { defineConfig } from 'vite';
import { resolve } from 'path';
import wasm from 'vite-plugin-wasm';

export default defineConfig(({ command }) => {
    const root = command === 'serve' ? './examples' : './';

    return {
        root,
        build: {
            outDir: './dist',
            target: 'esnext', // Установите target на esnext для поддержки top-level await
            rollupOptions: {
                input: {
                    main: resolve(__dirname, 'index.html'),
                },
                output: [
                    {
                        format: 'es',
                        entryFileNames: 'wasm-i18n.web.js',
                    },
                    {
                        format: 'cjs',
                        entryFileNames: 'wasm-i18n.node.js',
                    },
                ],
            },
        },
        server: {
            port: 9000,
        },
        plugins: [
            wasm(),
        ],
    };
});