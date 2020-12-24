const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const {CleanWebpackPlugin} = require("clean-webpack-plugin");

function html_generator(title, name) {
    return new HtmlWebpackPlugin({
        title: title,
        template: "./html/" + name + ".html",
        chunks: [name],
        filename: "./" + name + ".html",
        publicPath: "/"
    });
}

module.exports = {
    mode: "development",
    entry: {
        bouncing_balls: "./src/bouncing_balls.tsx",
        index: "./src/index.tsx",
        login: "./src/login.tsx",
        ruleset_builder: "./src/ruleset_builder.tsx",
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
        html_generator("Bouncing Balls", "bouncing_balls"),
        html_generator("Index", "index"),
        html_generator("Login", "login"),
        html_generator("RulesetBuilder", "ruleset_builder"),
    ]
};
