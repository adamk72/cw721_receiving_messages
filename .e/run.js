var info = require('../environment')
var exec = require('child_process').execSync;

const user = info.user
const chain = info.chain

const txFlag = info.txFlag.exec
const amtFlag = info.amtFlag.exec

const contract = info.store.contract

const arg = process.argv[2];
if (!arg) {
  console.warn("Require a JSON argument")
  process.exit(0);
}

const envWallet = process.env.WALLET;


const command = `wasmd tx wasm execute ${contract} '${arg}' ${amtFlag} --from ${envWallet || user.wallet} ${txFlag} -y`


console.log(command)
exec(command, { stdio: 'inherit' })

/*
 ***** HELPER INFO *****
 */

/* MINT
wasmd tx wasm execute $CONTRACT "$MINT" --amount 100umlg --from alice $TXFLAG
*/

const init = {
  "name": "Alice-1-NFT",
  "symbol": "ALICE",
  "minter": "wasm1z655rx0jlt5ujlgul2m8ufdj50jdjqlrydpumf"
}


/* Instantiate
wasmd tx wasm instantiate $CODE_ID "$INIT" --from alice --label "alice 2 nft" $TXFLAG -y --no-admin
*/

/* Store
RES=$(wasmd tx wasm store target/wasm32-unknown-unknown/release/cw721_base.wasm --from alice $TXFLAG -y --output json -b block)
*/