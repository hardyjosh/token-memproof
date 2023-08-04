<script lang="ts">
  import { onMount, setContext } from "svelte";
  import type { ActionHash, AppAgentClient, AppInfo } from "@holochain/client";
  import { AppAgentWebsocket } from "@holochain/client";
  import AllPosts from "./gated_dna/gated_dna/AllPosts.svelte";
  import CreatePost from "./gated_dna/gated_dna/CreatePost.svelte";
  import { Button, Heading, Input, Spinner } from "flowbite-svelte";

  import { clientContext } from "./contexts";
  import CreateClone from "./gated_dna/gated_dna/CreateClone.svelte";
  import AllLobbies from "./lobby/lobby/AllLobbies.svelte";

  import { configureChains } from "@wagmi/core";
  import { mainnet, polygon } from "@wagmi/core/chains";
  import { createConfig, account } from "svelte-wagmi-stores";
  // this example also uses Web3Modal - you'll need to install this yourself
  import { Web3Modal } from "@web3modal/html";
  import {
    EthereumClient,
    w3mConnectors,
    w3mProvider,
  } from "@web3modal/ethereum";

  // all this boilerplate is from the web3modal docs
  const chains = [mainnet];
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
  web3modal.setDefaultChain(mainnet);

  let client: AppAgentClient | undefined;
  let loading = true;
  let info: AppInfo | undefined;

  $: client, loading, info;

  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect("", "token-memproof");
    info = await client.appInfo();
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main>
  <span class="text-blue-500 text-2xl">Memproof</span>
  <Button on:click={web3modal.openModal}>Connect Wallet</Button>
  {#if loading}
    <div class="flex">
      <Spinner />
    </div>
  {:else if info}
    <!-- <pre>{JSON.stringify(info.cell_info, null, 2)}</pre> -->
    <span
      >{info.cell_info.gated_dna[0]?.provisioned.dna_modifiers.properties}</span
    >
    <AllPosts />
    <CreateClone />
    <AllLobbies />
  {/if}
</main>
