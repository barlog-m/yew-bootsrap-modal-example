const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    output: {
        path: path.join(__dirname, "dist"),
        publicPath: "/"
    },

    resolve: {
        modules: [path.resolve(__dirname, "pkg"), "node_modules"],
        extensions: [".js"]
    },

    module: {
        rules: [
            {
                test: /\.js$/,
                use: ["source-map-loader"],
                enforce: "pre",
            },
            {
                test: /\.css$/,
                use: [
                    MiniCssExtractPlugin.loader,
                    "css-loader"
                ]
            },
            {
                test: /\.(jpg|jpeg|gif|png|svg)$/,
                loader: "file-loader",
                options: {
                    limit: 1024,
                    name: "images/[name].[ext]?[hash]"
                }
            },
            {
                test: /\.(woff|woff2|eot|ttf)$/,
                loader: "file-loader",
                options: {
                    limit: 1024,
                    name: "fonts/[name].[ext]?[hash]"
                }
            }
        ]
    }
};
