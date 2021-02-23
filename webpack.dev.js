const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");

const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = merge(common, {
    mode: "development",

    entry: {
        bundle: "./index.js",
    },

    output: {
        filename: "js/[id].js",
    },

    plugins: [
        new MiniCssExtractPlugin({
            filename: "css/[name].css",
        }),
        new HtmlWebpackPlugin({
            template: "index.html"
        }),
        //new webpack.HotModuleReplacementPlugin(),
    ],

    devtool: "source-map",

    optimization: {
        minimize: false,
        emitOnErrors: true,
        removeAvailableModules: false,
        removeEmptyChunks: false,
        splitChunks: false,
    },

    devServer: {
        port: 8080,
        disableHostCheck: true,

        historyApiFallback: true,
        hot: true,

        proxy: {
            "/api/": {
                target: "http://localhost:8081/api",
                secure: false,
                prependPath: false
            }
        }
    }
});
