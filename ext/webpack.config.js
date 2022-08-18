const path = require('path');
const fs = require("fs");
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
   mode: "production",
   entry: fs.readdirSync('./src').reduce((acc, v) => ({ ...acc, [v.replace(/\.[^/.]+$/, "")]: path.resolve("src", v) }), {}),
   output: {
      path: path.join(__dirname, "./dist"),
      filename: "[name].js",
   },
   resolve: {
      extensions: [".ts", ".js"],
   },
   module: {
      rules: [
         {
            test: /\.tsx?$/,
            loader: "ts-loader",
            exclude: /node_modules/,
         },
      ],
   },
   plugins: [
      new CopyPlugin({
         patterns: [{from: ".", to: ".", context: "public"}]
      }),
   ],
};