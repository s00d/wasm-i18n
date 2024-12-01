const path = require('path');
const { DefinePlugin } = require('webpack');

const isDev = process.env.NODE_ENV !== 'production'

const targets = ['web', 'node'];

const createConfig = (target) => ({
    entry: './pkg/bundler/wasm_i18n.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: `wasm-i18n.${target}.js`,
        library: {
            name: 'wasmI18n',
            type: target === 'node' ? 'commonjs2' : 'umd',
        },
    },
    mode: 'production',
    devtool: 'source-map',
    plugins: [
        new DefinePlugin({
            'process.env.TARGET': JSON.stringify(target),
        }),
    ],
    resolve: {
        alias: {
            'wasm-i18n': path.resolve(__dirname, 'pkg'),
        },
    },
    devServer: {
        static: {
            directory: path.join(__dirname, 'examples'),
        },
        compress: true,
        port: 9000,
        hot: true,
    },
    experiments: {
        asyncWebAssembly: true,
    },
});

if (isDev) {
    module.exports = createConfig(['web']);
} else {
    module.exports = targets.map(createConfig);
}

