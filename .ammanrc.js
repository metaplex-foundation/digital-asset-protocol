// @ts-check
'use strict';
const path = require('path');
const {LOCALHOST, tmpLedgerDir } = require('@metaplex-foundation/amman');
const localDeployDir = path.join(__dirname, 'target', 'deploy');

const programIds = {
  dasset: 'assetbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
};

function localDeployPath(programName) {
  return path.join(localDeployDir, `${programName}.so`);
}
const programs = [
  { label: "Digital Asset Protocol", programId: programIds.dasset, deployPath: localDeployPath('mpl_asset') },
];

const validator = {
  killRunningValidators: true,
  programs,
  commitment: 'singleGossip',
  resetLedger: true,
  verifyFees: false,
  jsonRpcUrl: LOCALHOST,
  websocketUrl: '',
  ledgerDir: tmpLedgerDir(),
};

module.exports = {
  validator,
  relay: {
    enabled: true,
    killRunningRelay: true,
  },
};
