<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import type { AppAgentClient, Record } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { EvmKeyBinding } from "./types";
  import { Alert, Button } from "flowbite-svelte";
  import { formatAddress } from "../../lib/utils";
  import { IconOutline } from "svelte-heros-v2";
  import { account, walletClient } from "svelte-wagmi-stores";
  import { toBytes, toHex } from "viem";
  import { addSnackBar } from "../../lib/snackbar/snackbar";

  $: console.log($walletClient, $account);

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let evmKey: string = "";

  $: evmKey;

  enum EvmKeyBindingStatus {
    NotCreated,
    AwaitingSignature,
    Created,
  }

  let evmKeyBindingStatus: EvmKeyBindingStatus = EvmKeyBindingStatus.NotCreated;

  async function createEvmKeyBinding() {
    evmKeyBindingStatus = EvmKeyBindingStatus.AwaitingSignature;
    const sig = await $walletClient.signMessage({
      account: $account.address,
      message: { raw: toHex(client.myPubKey) },
    });

    const evmKeyBindingEntry: EvmKeyBinding = {
      evm_key: toBytes($account.address),
      signature_bytes: toBytes(sig),
    };

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: "lobby",
        zome_name: "lobby",
        fn_name: "create_evm_key_binding",
        payload: evmKeyBindingEntry,
      });
      dispatch("evm-key-binding-created", {
        evmKeyBindingHash: record.signed_action.hashed.hash,
      });
      evmKeyBindingStatus = EvmKeyBindingStatus.Created;
    } catch (e) {
      console.log(e);
      addSnackBar(`Error creating the evm key binding: ${e?.data?.data || e}`);
      // errorSnackbar.labelText = `Error creating the evm key binding: ${e.data.data}`;
      // errorSnackbar.show();
    }
  }
</script>

{#if $account?.isConnected}
  <div class="flex flex-col items-center justify-center pt-36">
    <div
      class="bg-white rounded-2xl p-6 max-w-md gap-y-6 items-center flex flex-col break-words"
    >
      <div class="flex flex-col gap-y-2 items-center">
        <div>You are binding EVM account</div>
        <div class="text-3xl text-gray-500 font-bold">
          {formatAddress($account.address)}
        </div>
        <!-- <IconOutline name="arrows-up-down" class="w-12 h-12 text-gray-500" /> -->
        <div>with your Holochain agent key</div>
        <div class="text-3xl text-gray-500 font-bold">
          {formatAddress(toHex(client.myPubKey))}
        </div>
      </div>
      <Button
        class="text-xl"
        disabled={!$account.isConnected}
        on:click={() => createEvmKeyBinding()}
        >Submit proof and bind EVM wallet to Holochain agent</Button
      >
      {#if evmKeyBindingStatus === EvmKeyBindingStatus.AwaitingSignature}
        <div class="text-blue-500">
          Please check your wallet and sign your Holochain agent key
        </div>
      {/if}
    </div>
  </div>
{/if}
