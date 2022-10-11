// /* eslint-disable */

module.exports = function () {
  const nodeEnv = process.env.NODE_ENV || 'development';
  return require(`./webpack.${nodeEnv}.js`);
};
