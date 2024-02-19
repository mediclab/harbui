const path = require('path');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    mode: 'production',
    entry: './js/index.js',
    output: {
        path: path.resolve(__dirname, '../public/js'),
        filename: 'app.js',
        assetModuleFilename: '../fonts/[name][ext][query]'
    },
    plugins: [new MiniCssExtractPlugin({
        filename: "../css/[name].css"
    })],
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
            },
            {
                test: /\.(woff|woff2|eot|ttf|otf)$/i,
                type: 'asset/resource',
            },
        ],
    },
};