

const ChainInfo = {
  // Chain-id of the Cosmos SDK chain.
  chainId: "malaga-420",
  // The name of the chain to be displayed to the user.
  chainName: "Malaga Testnet",
  // RPC endpoint of the chain.
  rpc: "https://rpc.malaga-420.cosmwasm.com:443",
  // REST endpoint of the chain.
  rest: "https://api.malaga-420.cosmwasm.com", // might be wrong
  // Staking coin information
  stakeCurrency: {
    // Coin denomination to be displayed to the user.
    coinDenom: "AND",
    // Actual denom (i.e. uatom, uscrt) used by the blockchain.
    coinMinimalDenom: "uand",
    // # of decimal points to convert minimal denomination to user-facing denomination.
    coinDecimals: 6,
    // (Optional) Keplr can show the fiat value of the coin if a coingecko id is provided.
    // You can get id from https://api.coingecko.com/api/v3/coins/list if it is listed.
    // coinGeckoId: ""
  },
  // (Optional) If you have a wallet webpage used to stake the coin then provide the url to the website in `walletUrlForStaking`.
  // The 'stake' button in Keplr extension will link to the webpage.
  // walletUrlForStaking: "",
  // The BIP44 path.
  bip44: {
    // You can only set the coin type of BIP44.
    // 'Purpose' is fixed to 44.
    coinType: 118,
  },
  // Bech32 configuration to show the address to user.
  bech32Config: {
    bech32PrefixAccAddr: "malaga",
    bech32PrefixAccPub: "malagapub",
    bech32PrefixValAddr: "malagavaloper",
    bech32PrefixValPub: "malagavaloperpub",
    bech32PrefixConsAddr: "malagavalcons",
    bech32PrefixConsPub: "malagavalconspub"
  },
  // List of all coin/tokens used in this chain.
  currencies: [{
    // Coin denomination to be displayed to the user.
    coinDenom: "MLG",
    // Actual denom (i.e. uatom, uscrt) used by the blockchain.
    coinMinimalDenom: "umlg",
    // # of decimal points to convert minimal denomination to user-facing denomination.
    coinDecimals: 6,
    // (Optional) Keplr can show the fiat value of the coin if a coingecko id is provided.
    // You can get id from https://api.coingecko.com/api/v3/coins/list if it is listed.
    // coinGeckoId: ""
  }],
  // List of coin/tokens used as a fee token in this chain.
  feeCurrencies: [{
    // Coin denomination to be displayed to the user.
    coinDenom: "MLG",
    // Actual denom (i.e. uatom, uscrt) used by the blockchain.
    coinMinimalDenom: "umlg",
    // # of decimal points to convert minimal denomination to user-facing denomination.
    coinDecimals: 6,
    // (Optional) Keplr can show the fiat value of the coin if a coingecko id is provided.
    // You can get id from https://api.coingecko.com/api/v3/coins/list if it is listed.
    // coinGeckoId: ""
  }],
  // (Optional) The number of the coin type.
  // This field is only used to fetch the address from ENS.
  // Ideally, it is recommended to be the same with BIP44 path's coin type.
  // However, some early chains may choose to use the Cosmos Hub BIP44 path of '118'.
  // So, this is separated to support such chains.
  coinType: 118,
  // (Optional) This is used to set the fee of the transaction.
  // If this field is not provided, Keplr extension will set the default gas price as (low: 0.01, average: 0.025, high: 0.04).
  // Currently, Keplr doesn't support dynamic calculation of the gas prices based on on-chain data.
  // Make sure that the gas prices are higher than the minimum gas prices accepted by chain validators and RPC/REST endpoint.
  gasPriceStep: {
    low: 0,
    average: 0.1,
    high: 0.2
  },
  faucets: ["https://faucet.malaga-420.cosmwasm.com"],
  features: ['cosmwasm']
}

const UserInfo = {
  wasmFile: "cw721_base.wasm",
  wallet: "wallet",
  gasAdjustment: "1.3",
}

const StoreInfo = {
  // CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
  codeId: "3240",
  // CONTRACT=$(wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
  contract: "wasm10ceey583s0pyft9vwpv9eskk5d35lac8zfgnslqjxglf0c35hfjs9jkcrl"
}

/**
 * Wallets:
 *  Alice:  wasm1z655rx0jlt5ujlgul2m8ufdj50jdjqlrydpumf
 *  Bob:    wasm1kmhmzgw7wvt9saaq8z2pk5aajxsznpz7c4l5m0
 *  Wallet: wasm1pa7r7e9uhdqzuxjejryt0yyrn55a5u5z5qqfwm
 */

/**
 * Contracts based on owner:
 *  Alice:  wasm16ndav9vfpejvq3u0xx657h264gt5rg56mmx4rz9s9tm0egxrhfyscems6q 
 *  Bob:    wasm1r2xm40aezdrtlxge6unmkzrq2ux372n27w9cl4w6jc7gvgtvyaxqf6kjz
 *  Wallet: wasm10ceey583s0pyft9vwpv9eskk5d35lac8zfgnslqjxglf0c35hfjs9jkcrl
 */

const GasPrice = {
  exec: "0.05" + ChainInfo.feeCurrencies[0].coinMinimalDenom,
  query: "100" + ChainInfo.feeCurrencies[0].coinMinimalDenom
}

const AmountPrice = {
  exec: "100" + ChainInfo.feeCurrencies[0].coinMinimalDenom,
  query: "100" + ChainInfo.feeCurrencies[0].coinMinimalDenom
}

const NodeFlag = `--node ${ChainInfo.rpc}`

const TxFlagBase = `${NodeFlag} --chain-id ${ChainInfo.chainId} --gas auto --gas-adjustment ${UserInfo.gasAdjustment}`


module.exports = {
  user: UserInfo,
  chain: ChainInfo,
  nodeFlag: NodeFlag,
  store: StoreInfo,
  txFlag: {
    query: `${TxFlagBase} --gas-prices ${GasPrice.query}`,
    exec: `${TxFlagBase} --gas-prices ${GasPrice.exec}`
  },
  amtFlag: {
    query: `--amount ${AmountPrice.query}`,
    exec: `--amount ${AmountPrice.query}` 
  }
};

/**
 * Setting up a contract on the command line:
 * 1. docker run --rm -v "$(pwd)":/code \\n  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \\n  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \\n  cosmwasm/workspace-optimizer:0.12.10
 3404  wasmd query wasm list-code $NODE
 *  - Use if starting from scratch or changes were made to contract
 * 2. RES=$(wasmd tx wasm store target/wasm32-unknown-unknown/release/cw721_base.wasm --from bob $TXFLAG -y --output json -b block)
 *  - Save $RES somewhere.
 * 3. CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
 * 4. INIT='{ "name": "Wall-1-NFT", "symbol": "WAL", "minter": "wasm1pa7r7e9uhdqzuxjejryt0yyrn55a5u5z5qqfwm" }'
 *  - Replace with appropriate names
 *  - or: INIT=$(cat .e/exec.json | jq '.initBob')
 * 5. wasmd tx wasm instantiate $CODE_ID "$INIT" --from wallet --label "name service" $TXFLAG -y --no-admin
 * 6. CONTRACT=$(wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
 *  - 
 */


// RES=$(wasmd tx wasm store artifacts/cw721_base.wasm --from alice $TXFLAG -y --output json -b block)
// This is run once with the following output (which will differ with each call since it's a new storage)
/* ORIGINAL ALICE RES 3216:
 {"height":"2921885","txhash":"43DD9A1CB8BACBF3DCF04542B59A635469A6B38A2D6F17260FDEB275DBB42E05","codespace":"","code":0,"data":"0A250A1E2F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F64651203089019","raw_log":"[{\"events\":[{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmwasm.wasm.v1.MsgStoreCode\"},{\"key\":\"module\",\"value\":\"wasm\"},{\"key\":\"sender\",\"value\":\"wasm1z655rx0jlt5ujlgul2m8ufdj50jdjqlrydpumf\"}]},{\"type\":\"store_code\",\"attributes\":[{\"key\":\"code_id\",\"value\":\"3216\"}]}]}]","logs":[{"msg_index":0,"log":"","events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1z655rx0jlt5ujlgul2m8ufdj50jdjqlrydpumf"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"3216"}]}]}],"info":"","gas_wanted":"2426669","gas_used":"1880804","tx":null,"timestamp":"","events":[{"type":"coin_spent","attributes":[{"key":"c3BlbmRlcg==","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true},{"key":"YW1vdW50","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"coin_received","attributes":[{"key":"cmVjZWl2ZXI=","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"YW1vdW50","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"transfer","attributes":[{"key":"cmVjaXBpZW50","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"c2VuZGVy","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true},{"key":"YW1vdW50","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"message","attributes":[{"key":"c2VuZGVy","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true}]},{"type":"tx","attributes":[{"key":"ZmVl","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"tx","attributes":[{"key":"YWNjX3NlcQ==","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZi8z","index":true}]},{"type":"tx","attributes":[{"key":"c2lnbmF0dXJl","value":"N2VzakVCeHoxK1N0UjVGRW85TTNwcFBBN3J1UkVNQVlWdE1PUjFuQlhhQitnRHYrVmUrYS9RdjVrUG1uMXBZd3NHMVlOcnByU2c3b0Rlbk45eEJKYWc9PQ==","index":true}]},{"type":"message","attributes":[{"key":"YWN0aW9u","value":"L2Nvc213YXNtLndhc20udjEuTXNnU3RvcmVDb2Rl","index":true}]},{"type":"message","attributes":[{"key":"bW9kdWxl","value":"d2FzbQ==","index":true},{"key":"c2VuZGVy","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true}]},{"type":"store_code","attributes":[{"key":"Y29kZV9pZA==","value":"MzIxNg==","index":true}]}]}
 */

/* Alternate ALICE RES:
{"height":"2977962","txhash":"44E541D91F14E74D8A4E1F61BB620FC58F37F5C46989862C6508416A72DAFA4C","codespace":"","code":0,"data":"0A250A1E2F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F6465120308A619","raw_log":"[{\"events\":[{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmwasm.wasm.v1.MsgStoreCode\"},{\"key\":\"module\",\"value\":\"wasm\"},{\"key\":\"sender\",\"value\":\"wasm1z655rx0jlt5ujlgul2m8ufdj50jdjqlrydpumf\"}]},{\"type\":\"store_code\",\"attributes\":[{\"key\":\"code_id\",\"value\":\"3238\"}]}]}]","logs":[{"msg_index":0,"log":"","events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1z655rx0jlt5ujlgul2m8ufdj50jdjqlrydpumf"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"3238"}]}]}],"info":"","gas_wanted":"2184062","gas_used":"1694183","tx":null,"timestamp":"","events":[{"type":"coin_spent","attributes":[{"key":"c3BlbmRlcg==","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true},{"key":"YW1vdW50","value":"MTA5MjA0dW1sZw==","index":true}]},{"type":"coin_received","attributes":[{"key":"cmVjZWl2ZXI=","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"YW1vdW50","value":"MTA5MjA0dW1sZw==","index":true}]},{"type":"transfer","attributes":[{"key":"cmVjaXBpZW50","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"c2VuZGVy","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true},{"key":"YW1vdW50","value":"MTA5MjA0dW1sZw==","index":true}]},{"type":"message","attributes":[{"key":"c2VuZGVy","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true}]},{"type":"tx","attributes":[{"key":"ZmVl","value":"MTA5MjA0dW1sZw==","index":true}]},{"type":"tx","attributes":[{"key":"YWNjX3NlcQ==","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZi82","index":true}]},{"type":"tx","attributes":[{"key":"c2lnbmF0dXJl","value":"Z2Z6THkxL0VXeVFsQmlGNW5kSGlRN2dJdHFHd3NKeGt3MFBDcEp1am8xb1A0bFpKdXF2Qnl1dWV4U1dXTXQxMGRtemx3VmVnUGVDTnRWMHY1ZEFDd0E9PQ==","index":true}]},{"type":"message","attributes":[{"key":"YWN0aW9u","value":"L2Nvc213YXNtLndhc20udjEuTXNnU3RvcmVDb2Rl","index":true}]},{"type":"message","attributes":[{"key":"bW9kdWxl","value":"d2FzbQ==","index":true},{"key":"c2VuZGVy","value":"d2FzbTF6NjU1cngwamx0NXVqbGd1bDJtOHVmZGo1MGpkanFscnlkcHVtZg==","index":true}]},{"type":"store_code","attributes":[{"key":"Y29kZV9pZA==","value":"MzIzOA==","index":true}]}]}
 */

/* NEW CHECK-CONFIRM BOB RES 3239 (Might be corrupt because of add_attribute?):
{"height":"2981322","txhash":"99D7AB123A756F9AAA1E60B924C81385E3CDB30D9A23867F69293001536C8E6F","codespace":"","code":0,"data":"0A250A1E2F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F6465120308A719","raw_log":"[{\"events\":[{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmwasm.wasm.v1.MsgStoreCode\"},{\"key\":\"module\",\"value\":\"wasm\"},{\"key\":\"sender\",\"value\":\"wasm1kmhmzgw7wvt9saaq8z2pk5aajxsznpz7c4l5m0\"}]},{\"type\":\"store_code\",\"attributes\":[{\"key\":\"code_id\",\"value\":\"3239\"}]}]}]","logs":[{"msg_index":0,"log":"","events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1kmhmzgw7wvt9saaq8z2pk5aajxsznpz7c4l5m0"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"3239"}]}]}],"info":"","gas_wanted":"2426669","gas_used":"1880804","tx":null,"timestamp":"","events":[{"type":"coin_spent","attributes":[{"key":"c3BlbmRlcg==","value":"d2FzbTFrbWhtemd3N3d2dDlzYWFxOHoycGs1YWFqeHN6bnB6N2M0bDVtMA==","index":true},{"key":"YW1vdW50","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"coin_received","attributes":[{"key":"cmVjZWl2ZXI=","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"YW1vdW50","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"transfer","attributes":[{"key":"cmVjaXBpZW50","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"c2VuZGVy","value":"d2FzbTFrbWhtemd3N3d2dDlzYWFxOHoycGs1YWFqeHN6bnB6N2M0bDVtMA==","index":true},{"key":"YW1vdW50","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"message","attributes":[{"key":"c2VuZGVy","value":"d2FzbTFrbWhtemd3N3d2dDlzYWFxOHoycGs1YWFqeHN6bnB6N2M0bDVtMA==","index":true}]},{"type":"tx","attributes":[{"key":"ZmVl","value":"MTIxMzM0dW1sZw==","index":true}]},{"type":"tx","attributes":[{"key":"YWNjX3NlcQ==","value":"d2FzbTFrbWhtemd3N3d2dDlzYWFxOHoycGs1YWFqeHN6bnB6N2M0bDVtMC8y","index":true}]},{"type":"tx","attributes":[{"key":"c2lnbmF0dXJl","value":"M2htZXBIQ0l0YU1aSEdRNTRxSUxPZHM1RG5LS1F3WXZuc2doYlludTQvbGRwY1Nsc2E4YnN2TVdvR1ZCMXNHcnE2WVFnMnZRTjFYUzhzUmV4bTRVemc9PQ==","index":true}]},{"type":"message","attributes":[{"key":"YWN0aW9u","value":"L2Nvc213YXNtLndhc20udjEuTXNnU3RvcmVDb2Rl","index":true}]},{"type":"message","attributes":[{"key":"bW9kdWxl","value":"d2FzbQ==","index":true},{"key":"c2VuZGVy","value":"d2FzbTFrbWhtemd3N3d2dDlzYWFxOHoycGs1YWFqeHN6bnB6N2M0bDVtMA==","index":true}]},{"type":"store_code","attributes":[{"key":"Y29kZV9pZA==","value":"MzIzOQ==","index":true}]}]}
*/

/* WALLET RES 3240:
{"height":"2982315","txhash":"D0A0C48C2FE77A24F9256A746D90EB750D6440B7B9F41BCD910EBE8BFBA9F812","codespace":"","code":0,"data":"0A250A1E2F636F736D7761736D2E7761736D2E76312E4D736753746F7265436F6465120308A819","raw_log":"[{\"events\":[{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmwasm.wasm.v1.MsgStoreCode\"},{\"key\":\"module\",\"value\":\"wasm\"},{\"key\":\"sender\",\"value\":\"wasm1pa7r7e9uhdqzuxjejryt0yyrn55a5u5z5qqfwm\"}]},{\"type\":\"store_code\",\"attributes\":[{\"key\":\"code_id\",\"value\":\"3240\"}]}]}]","logs":[{"msg_index":0,"log":"","events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"wasm1pa7r7e9uhdqzuxjejryt0yyrn55a5u5z5qqfwm"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"3240"}]}]}],"info":"","gas_wanted":"2435257","gas_used":"1887380","tx":null,"timestamp":"","events":[{"type":"coin_spent","attributes":[{"key":"c3BlbmRlcg==","value":"d2FzbTFwYTdyN2U5dWhkcXp1eGplanJ5dDB5eXJuNTVhNXU1ejVxcWZ3bQ==","index":true},{"key":"YW1vdW50","value":"MTIxNzYzdW1sZw==","index":true}]},{"type":"coin_received","attributes":[{"key":"cmVjZWl2ZXI=","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"YW1vdW50","value":"MTIxNzYzdW1sZw==","index":true}]},{"type":"transfer","attributes":[{"key":"cmVjaXBpZW50","value":"d2FzbTE3eHBmdmFrbTJhbWc5NjJ5bHM2Zjg0ejNrZWxsOGM1bDY5ajR6aw==","index":true},{"key":"c2VuZGVy","value":"d2FzbTFwYTdyN2U5dWhkcXp1eGplanJ5dDB5eXJuNTVhNXU1ejVxcWZ3bQ==","index":true},{"key":"YW1vdW50","value":"MTIxNzYzdW1sZw==","index":true}]},{"type":"message","attributes":[{"key":"c2VuZGVy","value":"d2FzbTFwYTdyN2U5dWhkcXp1eGplanJ5dDB5eXJuNTVhNXU1ejVxcWZ3bQ==","index":true}]},{"type":"tx","attributes":[{"key":"ZmVl","value":"MTIxNzYzdW1sZw==","index":true}]},{"type":"tx","attributes":[{"key":"YWNjX3NlcQ==","value":"d2FzbTFwYTdyN2U5dWhkcXp1eGplanJ5dDB5eXJuNTVhNXU1ejVxcWZ3bS8w","index":true}]},{"type":"tx","attributes":[{"key":"c2lnbmF0dXJl","value":"N2NBK0hOK2V6WWVGWUdFZ1IvUjlUYzQ1V29uR3pZeVRZQXV6dmk3aDAxWk81SFVGUnMyQzNnN2tLeVA2ck1KZVhJNXZ0ci9YaCtGa0ZmN3AzeURDZVE9PQ==","index":true}]},{"type":"message","attributes":[{"key":"YWN0aW9u","value":"L2Nvc213YXNtLndhc20udjEuTXNnU3RvcmVDb2Rl","index":true}]},{"type":"message","attributes":[{"key":"bW9kdWxl","value":"d2FzbQ==","index":true},{"key":"c2VuZGVy","value":"d2FzbTFwYTdyN2U5dWhkcXp1eGplanJ5dDB5eXJuNTVhNXU1ejVxcWZ3bQ==","index":true}]},{"type":"store_code","attributes":[{"key":"Y29kZV9pZA==","value":"MzI0MA==","index":true}]}]}
*/
