const path = require('path');
const autoPreprocess =  require('svelte-preprocess');
const { ProvidePlugin } = require('webpack');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const entryPath = path.resolve(__dirname, 'src', 'main.js');
const htmlPath = path.resolve(__dirname, 'src', 'index.html');
const outputPath = path.resolve(__dirname, 'dist');

const aliasSveltePath = path.resolve('node_modules', 'svelte');
const aquaPath = path.resolve(__dirname, 'src', '_aqua');
const aliasComponentsPath = path.resolve(__dirname, 'src', 'components');
const aliasHelpersPath = path.resolve(__dirname, 'src', 'helpers');
const aliasStorePath = path.resolve(__dirname, 'src', 'store');
const aliasTypesPath = path.resolve(__dirname, 'src', 'types');

module.exports = {
  entry: entryPath,
  mode: 'development',
  output: {
    path: outputPath,
    filename: 'main.js',
  },
  resolve: {
    extensions: ['.mjs', '.js', '.svelte', '.wasm'],
    mainFields: ['svelte', 'browser', 'module', 'main'],
    alias: {
      svelte: aliasSveltePath,
      '@aqua': aquaPath,
      '@components': aliasComponentsPath,
      '@helpers': aliasHelpersPath,
      '@store': aliasStorePath,
      '@types': aliasTypesPath,
    },
  },
  module: {
    rules: [
      {
        test: /\.(html|svelte)$/,
        use: {
          loader: 'svelte-loader',
          options: {
            preprocess: autoPreprocess(),
            emitCss: true,
          },
        },
      },
      {
        test: /\.js$/,
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
      {
        test: /node_modules\/svelte\/.*\.mjs$/,
        resolve: {
          fullySpecified: false,
        },
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
    historyApiFallback: true,
  },
};
