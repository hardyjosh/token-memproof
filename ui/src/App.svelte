<script lang="ts">
  import { toHex } from "viem";
  import { onMount, setContext } from "svelte";
  import {
    encodeHashToBase64,
    type AppAgentClient,
    type CellId,
  } from "@holochain/client";
  import { AppAgentWebsocket } from "@holochain/client";
  import AllPosts from "./gated_dna/gated_dna/AllPosts.svelte";
  import { Spinner } from "flowbite-svelte";

  import { clientContext, currentGatedClone } from "./contexts";
  import AllLobbies from "./lobby/lobby/AllLobbies.svelte";

  import { configureChains } from "@wagmi/core";
  import { mainnet, polygon, polygonMumbai } from "@wagmi/core/chains";
  import { createConfig, account } from "svelte-wagmi-stores";
  // this example also uses Web3Modal - you'll need to install this yourself
  import { Web3Modal } from "@web3modal/html";
  import {
    EthereumClient,
    w3mConnectors,
    w3mProvider,
  } from "@web3modal/ethereum";
  import Snackbar from "./lib/snackbar/Snackbar.svelte";
  import CreateEvmKeyBinding from "./lobby/lobby/CreateEvmKeyBinding.svelte";
  import ConnectWallet from "./ConnectWallet.svelte";
  import { getEvmKeyBinding } from "./lib/evm_key_binding";
  import type { EvmKeyBinding } from "./lobby/lobby/types";

  // all this boilerplate is from the web3modal docs
  const chains = [polygonMumbai];
  const projectId = "ed03fa277413d0dc05fcc318b714c759";

  const { publicClient } = configureChains(chains, [
    w3mProvider({ projectId }),
  ]);

  // except here we're using createConfig form this package instead of wagmi
  const wagmiConfig = createConfig({
    autoConnect: false,
    connectors: w3mConnectors({ projectId, chains }),
    publicClient,
  });

  const ethereumClient = new EthereumClient(wagmiConfig, chains);

  let web3modal: Web3Modal;
  web3modal = new Web3Modal({ projectId }, ethereumClient);
  web3modal.setDefaultChain(polygonMumbai);

  let client: AppAgentClient | undefined;
  let loading = true;
  let evmKeyBinding: EvmKeyBinding | undefined;

  $: client, loading;

  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect("", "token-memproof");
    ({ evmKeyBinding } = await getEvmKeyBinding(client));
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main class="relative w-full">
  <div class="flex justify-between p-4 border-b">
    <span class="text-blue-500 text-2xl">Memproof</span>
    <div class="flex flex-row text-sm gap-x-4">
      <div class="flex flex-col">
        <span>Bound EVM key</span>
        <span>
          {#if evmKeyBinding}
            {toHex(evmKeyBinding.evm_key)}
          {:else}
            None
          {/if}
        </span>
      </div>
      <div class="flex flex-col">
        <span>Agent pubkey</span>
        {#if client?.myPubKey}
          <span>{encodeHashToBase64(client.myPubKey)}</span>
        {/if}
      </div>
    </div>
  </div>
  {#if loading}
    <Spinner />
  {:else if $currentGatedClone}
    <AllPosts cellId={$currentGatedClone} />
  {:else if !$account?.isConnected}
    <ConnectWallet {web3modal} />
  {:else if !evmKeyBinding}
    <CreateEvmKeyBinding />
  {:else}
    <AllLobbies />
  {/if}
  <Snackbar />
</main>
