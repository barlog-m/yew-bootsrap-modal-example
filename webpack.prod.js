const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");
const path = require("path");

const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = merge(common, {
    mode: "production",

    entry: {
        bundle: "./index.js",
    },

    output: {
        filename: "js/[name].[chunkhash].js"
    },

    plugins: [
        new CleanWebpackPlugin(),
        new MiniCssExtractPlugin({
            filename: "css/[name].[chunkhash].css",
        }),
        new HtmlWebpackPlugin({
            template: "index.html",
            hash: true,
            minify: {
                removeComments: true,
                collapseWhitespace: true
            }
        }),
        new CopyPlugin({
            patterns: [
                {
                    from: path.resolve(__dirname, "pkg", "*.wasm"),
                    to: "pkg/[name].wasm",
                },
            ],
        })
    ],

    devtool: "source-map",

    optimization: {
        usedExports: true,
        emitOnErrors: false,
        runtimeChunk: "single",
        splitChunks: {
            cacheGroups: {
                vendor: {
                    test: /[\\/]node_modules[\\/]/,
                    name: "vendor",
                    chunks: "all"
                }
            }
        },
        minimize: true
    }
});
