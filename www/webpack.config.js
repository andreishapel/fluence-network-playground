const path = require('path');
const autoPreprocess =  require('svelte-preprocess');
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
    extensions: ['.js', '.svelte', '.wasm'],
    mainFields: ['svelte', 'browser', 'module', 'main'],
  },
  module: {
    rules: [
      {
        test: /\.svelte$/,
        use: {
          loader: 'svelte-loader',
          options: {
            preprocess: autoPreprocess(),
          },
        },
      },
      {
        test: /\.js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env'],
          },
        },
      },
      {
        test: /\.css$/,
        use: [
          'style-loader',
          'css-loader',
        ],
      },
      {
        test: /\.scss$/,
        use: [
          'style-loader',
          'css-loader',
          'sass-loader',
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
