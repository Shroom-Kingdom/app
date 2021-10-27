// /* eslint-disable */

module.exports = function (env) {
  const nodeEnv = process.env.NODE_ENV || 'development';
  return require(`./webpack.${nodeEnv}.js`);
};
