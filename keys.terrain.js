// can use `process.env.SECRET_MNEMONIC` or `process.env.SECRET_PRIV_KEY`
// to populate secret in CI environment instead of hardcoding

module.exports = {
  dexterdev8: {
    mnemonic:
      process.env.PRIVATE_KEY,
  },
};
