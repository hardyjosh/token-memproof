<script lang="ts">
  import type { Web3Modal } from "@web3modal/html";
  import { Button } from "flowbite-svelte";

  export let web3modal: Web3Modal;
</script>

<Button on:click={web3modal.openModal}>Connect Wallet</Button>
