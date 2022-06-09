const path = require('path');
const { ProvidePlugin } = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const entryPath = path.resolve(__dirname, 'src', 'main.js');
const htmlPath = path.resolve(__dirname, 'src', 'index.html');
const outputPath = path.resolve(__dirname, 'dist');

module.exports = {
  entry: entryPath,
  mode: 'development',
  output: {
    path: outputPath,
    filename: 'main.js',
  },
  resolve: {
    extensions: ['.js', '.svelte'],
    mainFields: ['svelte', 'browser', 'module', 'main'],
  },
  module: {
    rules: [
      {
        test: /\.svelte$/,
        use: {
          loader: 'svelte-loader',
        },
      },
      {
        test: /\.css$/,
        use: [
          'style-loader',
          'css-loader',
        ],
      },
    ],
  },
  plugins: [
    new ProvidePlugin({
      process: 'process/browser',
    }),
    new CopyWebpackPlugin({
      patterns: [{
        from: htmlPath,
        to: 'index.html',
      }],
    }),
  ],
  devServer: {
    hot: true,
  },
};
