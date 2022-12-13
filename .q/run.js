var info = require('../environment')
var exec = require('child_process').execSync;

const contract = info.store.contract

const arg = process.argv[2];
if (!arg) {
  console.warn("Require a JSON argument")
  process.exit(0);
}

const command = `wasmd query wasm contract-state smart ${contract} '${arg}' ${info.nodeFlag}  --output json`

console.log(command)
exec(command, { stdio: 'inherit' })

/* BANK BALANCES
wasmd query bank balances $(wasmd keys show -a alice) $NODE
*/
