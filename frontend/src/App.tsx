import { Component, createSignal, onMount } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { MalagaInfo } from "../MalagaInfo"

const App: Component = () => {
  onMount(async () => {
    const client = await CosmWasmClient.connect(MalagaInfo.rpc)
    setContract(await client.getContracts(3216))
  })
  const [contractId, setContract] = createSignal();

  return (
    <div class={styles.App}>
      <main>
        <pre>Contract: {contractId() as string || "awaiting..."}</pre>
      </main>
    </div>
  );
};

export default App;
