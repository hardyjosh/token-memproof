<script lang="ts">
  import type { Web3Modal } from "@web3modal/html";
  import { Button, Heading } from "flowbite-svelte";

  export let web3modal: Web3Modal;
</script>

<div class="flex flex-col items-center gap-y-4 mt-36">
  <Heading class="self-center text-center" tag="h4"
    >Connect your wallet to get started</Heading
  >
  <Button on:click={web3modal.openModal}>Connect Wallet</Button>
</div>
