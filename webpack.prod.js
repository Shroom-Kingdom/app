/* eslint-disable */

const HtmlWebpackPlugin = require('html-webpack-plugin');
const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');
const webpack = require('webpack');

const dist = path.resolve(__dirname, 'dist');

const BundleAnalyzerPlugin = require('webpack-bundle-analyzer')
  .BundleAnalyzerPlugin;

module.exports = {
  mode: 'production',
  entry: './src/index.tsx',
  output: {
    path: dist,
    filename: 'bundle.js'
  },
  devtool: 'source-map',
  plugins: [
    new HtmlWebpackPlugin({
      template: 'src/index.html'
    }),
    new ScriptExtHtmlWebpackPlugin({
      defaultAttribute: 'defer'
    }),
    new CopyPlugin({
      patterns: [
        { from: './src/static', to: '.' },
        { from: './pkg/app.js', to: '.' },
        { from: './pkg/app_bg.wasm', to: '.' }
      ],
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'production'
    }),
    new BundleAnalyzerPlugin({
      analyzerMode: 'static',
      reportFilename: path.join(__dirname, 'bundle-report.html'),
      openAnalyzer: false,
      generateStatsFile: true,
      statsFilename: path.join(__dirname, 'stats.json')
    })
  ],
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.jsx', '.json', '.wasm']
  },
  experiments: {
    asyncWebAssembly: true
  },
  externals: [
    ({ request }, callback) => {
      if (/\.wasm$/.test(request)) {
        return callback(null, "amd " + request);
      }
      callback();
    }
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: ['babel-loader']
      },
      {
        test: /\.svg$/,
        use: ['@svgr/webpack'],
      },
      {
        test: /\.(jpe?g|png|gif)$/i, 
        use: 'file-loader'
      }
    ]
  }
};
