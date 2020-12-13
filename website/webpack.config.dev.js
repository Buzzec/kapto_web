const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const {CleanWebpackPlugin} = require("clean-webpack-plugin");

module.exports = {
    mode: "development",
    entry: {
        index: "./src/index.ts",
        chat: "./src/chat.ts",
        canvas: "./src/canvas.ts"
    },
    devtool: "inline-source-map",
    devServer: {
        contentBase: "./dist",
    },
    output: {
        filename: "[name].bundle.js",
        path: path.resolve(__dirname, "dist"),
    },
    module: {
        rules: [
            {test: /\.css$/, use: ["style-loader", "css-loader"]},
            {test: /\.(png|svg|jpe?g|gif)$/i, loader: 'file-loader', options: {name: "assets/[hash].[ext]"}},
            {test: /\.tsx?$/, use: 'ts-loader', exclude: /node_modules/},
        ]
    },
    resolve: {
        extensions: [".ts", ".tsx", ".js"]
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            title: "Index",
            template: "./html/index.html",
            chunks: ["index"],
            filename: "./index.html",
            publicPath: "/web"
        }),
        new HtmlWebpackPlugin({
            title: "Chat",
            template: "./html/chat.html",
            chunks: ["chat"],
            filename: "./chat.html",
            publicPath: "/web"
        }),
        new HtmlWebpackPlugin({
            title: "Canvas",
            template: "./html/canvas.html",
            chunks: ["canvas"],
            filename: "./canvas.html",
            publicPath: "/web"
        })
    ]
};
