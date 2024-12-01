const path = require('path');
const { DefinePlugin } = require('webpack');

module.exports = {
    entry: './pkg/bundler/wasm_i18n.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: `wasm-i18n.web.js`,
        library: {
            name: 'wasmI18n',
            type: 'umd',
        },
    },
    mode: 'production',
    devtool: 'source-map',
    plugins: [
        new DefinePlugin({
            'process.env.TARGET': JSON.stringify('web'),
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
};