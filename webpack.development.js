/* eslint-disable */

const HtmlWebpackPlugin = require('html-webpack-plugin');
const SveltePreprocess = require('svelte-preprocess');
const Autoprefixer = require('autoprefixer');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');
const webpack = require('webpack');

const dist = path.resolve(__dirname, 'dist');

module.exports = {
  mode: 'development',
  entry: './src/main.js',
  output: {
    path: dist,
    filename: 'bundle.js'
  },
  devtool: 'inline-source-map',
  plugins: [
    new HtmlWebpackPlugin({
      template: 'src/index.html'
    }),
    new CopyPlugin({
      patterns: [
        { from: './src/static', to: '.' },
        { from: './pkg/app.js', to: '.' },
        { from: './pkg/app_bg.wasm', to: '.' }
      ],
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'development'
    }),
		new MiniCssExtractPlugin({
			filename: '[name].css'
		})
  ],
  resolve: {
		alias: {
			// Note: Later in this config file, we'll automatically add paths from `tsconfig.compilerOptions.paths`
			svelte: path.resolve('node_modules', 'svelte')
		},
    extensions: ['.ts', '.tsx', '.js', '.jsx', '.mjs', '.svelte', '.json', '.wasm'],
		mainFields: ['svelte', 'browser', 'module', 'main']
  },
  experiments: {
    asyncWebAssembly: true
  },
  watchOptions: {
    aggregateTimeout: 200,
    ignored: ['./target/**', './src-wasm/**']
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
        test: /\.ts$/,
        exclude: /node_modules/,
        use: ['babel-loader']
      },
			{
				test: /\.svelte$/,
				exclude: /node_modules/,
				use: {
					loader: 'svelte-loader',
					options: {
						compilerOptions: {
							dev: true
						},
						hotReload: true,
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
