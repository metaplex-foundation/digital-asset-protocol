module.exports = function override(config, env) {
  config.resolve.fallback = {
    fs: false,
    crypto: false,
    buffer:false,
    stream: false,
    assert: require.resolve("assert"),
  };
  return config;
};