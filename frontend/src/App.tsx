import { Component, createSignal, For, onMount, Show } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { MalagaInfo } from "../MalagaInfo"

const App: Component = () => {
  onMount(async () => {
    const client = await CosmWasmClient.connect(MalagaInfo.rpc)
    const id = (await client.getContracts(3216)).toString()

    let tokensA;
    try {
      const {tokens} = await client.queryContractSmart(id, { "tokens": { "owner": "wasm1kmhmzgw7wvt9saaq8z2pk5aajxsznpz7c4l5m0" } })
      tokensA = tokens
    } catch (error) {
      setError((error as Error).message)
    }
    setTokens(tokensA)
  })
  const [tokens, setTokens] = createSignal();
  const [error, setError] = createSignal();

  return (
    <div class={styles.App}>
      <main >
        <pre>Contract: {<For each={tokens() as string[]}>{(token, i) =>
          <li>
            {i() + 1}: {token}
          </li>
        }</For>}</pre>
        <Show when={error()}><pre>Error: {error() as string} </pre></Show>
      </main>
    </div>
  );
};

export default App;
