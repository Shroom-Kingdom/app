/* eslint-disable */

const HtmlWebpackPlugin = require('html-webpack-plugin');
const SveltePreprocess = require('svelte-preprocess');
const Autoprefixer = require('autoprefixer');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const CopyPlugin = require('copy-webpack-plugin');
const NodePolyfillPlugin = require('node-polyfill-webpack-plugin');
const path = require('path');
const webpack = require('webpack');

const dist = path.resolve(__dirname, 'dist');

const BundleAnalyzerPlugin = require('webpack-bundle-analyzer')
  .BundleAnalyzerPlugin;

module.exports = {
  mode: 'production',
  entry: './src/main.js',
  output: {
    path: dist,
    filename: 'bundle.js'
  },
  devtool: 'source-map',
  plugins: [
    new HtmlWebpackPlugin({
      template: 'src/index.html'
    }),
    new CopyPlugin({
      patterns: [
        { from: './src/static', to: '.' },
        { from: './assets', to: './assets' },
        { from: './pkg/app.js', to: '.' },
        { from: './pkg/app_bg.wasm', to: '.' }
      ],
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'production'
    }),
		new MiniCssExtractPlugin({
			filename: '[name].css'
		}),
		new NodePolyfillPlugin(),
    new BundleAnalyzerPlugin({
      analyzerMode: 'static',
      reportFilename: path.join(__dirname, 'bundle-report.html'),
      openAnalyzer: false,
      generateStatsFile: true,
      statsFilename: path.join(__dirname, 'stats.json')
    })
  ],
  resolve: {
		alias: {
			svelte: path.resolve('..', 'node_modules', 'svelte')
		},
    extensions: ['.ts', '.tsx', '.js', '.jsx', '.mjs', '.svelte', '.json', '.wasm'],
		mainFields: ['svelte', 'browser', 'module', 'main']
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
  optimization: {
    minimizer: [
      `...`,
      new CssMinimizerPlugin(),
    ],
  },
  module: {
    rules: [
      {
        test: /\.(jpe?g|png|gif|svg|webp)$/i,
        type: "asset",
      },
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: ['babel-loader']
      },
			{
				test: /\.svelte$/,
				use: {
					loader: 'svelte-loader',
					options: {
						emitCss: true,
						preprocess: SveltePreprocess({
							scss: true,
							sass: true,
							postcss: {
								plugins: [
									Autoprefixer
								]
							}
						})
					}
				}
			},
			{
				test: /node_modules\/svelte\/.*\.mjs$/,
				resolve: {
					fullySpecified: false
				}
			},
			{
				test: /\.(scss|sass)$/,
				use: [
					{
						loader: MiniCssExtractPlugin.loader
					},
					'css-loader',
					{
						loader: 'postcss-loader',
						options: {
							postcssOptions: {
								plugins: [
									Autoprefixer
								]
							}
						}
					},
					'sass-loader'
				]
			},
			{
				test: /\.css$/,
				use: [
					{
						loader: MiniCssExtractPlugin.loader
					},
					'css-loader',
				]
			},
      {
        test: /\.(jpe?g|png|gif)$/i, 
        use: 'file-loader'
      }
    ]
  }
};
