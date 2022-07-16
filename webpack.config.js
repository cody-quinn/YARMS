const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  mode: "production",
  entry: {
    create_repository: "./assets/src/typescript/createRepository.ts",
  },
  output: {
    path: path.resolve(__dirname, "assets/dist"),
    filename: "[name].bundle.js",
    assetModuleFilename: "[name][ext]",
    clean: true,
  },
  module: {
    rules: [
      {
        test: /\.ts$/i,
        use: "ts-loader",
      },
    ],
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        {
          context: "assets/src/",
          from: "./**/*.(css|png|jpg|jpeg|gif|ico)",
          to: "../dist/[path][name][ext]",
        },
      ],
    }),
  ],
  resolve: {
    extensions: [".ts"],
  },
  cache: {
    type: "filesystem",
    allowCollectingMemory: true,
  },
};
