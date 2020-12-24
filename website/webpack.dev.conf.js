const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const {CleanWebpackPlugin} = require("clean-webpack-plugin");

module.exports = {
    mode: "development",
    entry: {
        index: "./src/index.tsx",
        ruleset_builder: "./src/ruleset_builder.tsx",
        chat: "./src/chat.ts",
        bouncing_balls: "./src/bouncing_balls.tsx"
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
            {test: /\.tsx?$/, use: 'babel-loader', exclude: /node_modules/},
            {test: /\.js$/, use: ["source-map-loader"], enforce: "pre"}
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
            publicPath: "/"
        }),
        new HtmlWebpackPlugin({
            title: "RulesetBuilder",
            template: "./html/ruleset_builder.html",
            chunks: ["ruleset_builder"],
            filename: "./ruleset_builder.html",
            publicPath: "/"
        }),
        new HtmlWebpackPlugin({
            title: "Bouncing Balls",
            template: "./html/bouncing_balls.html",
            chunks: ["bouncing_balls"],
            filename: "./bouncing_balls.html",
            publicPath: "/"
        })
    ]
};
