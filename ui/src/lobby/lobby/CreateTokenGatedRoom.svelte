<script lang="ts">
  import type { TokenGatedRoom } from "./types.js";
  import { Button, Heading, Input } from "flowbite-svelte";
  import { clientContext } from "../../contexts.js";
  import type { AppAgentClient, Record } from "@holochain/client";
  import { createEventDispatcher, getContext } from "svelte";
  import { decode, encode } from "@msgpack/msgpack";
  import {
    getAddress,
    numberToBytes,
    numberToHex,
    parseUnits,
    toBytes,
  } from "viem";
  import type { AppProperties } from "../../gated_dna/gated_dna/types.js";
  import { fetchToken } from "@wagmi/core";
  import { parse } from "svelte/compiler";

  const dispatch = createEventDispatcher();

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let appProperties: AppProperties = {
    name: "hi",
    signer: "0x3cDB3d9e1B74692Bb1E3bb5fc81938151cA64b02",
    token: "0x2Eb1D24aB0eC5FD0058ab5073F1EA2d8A59783E5",
    threshold: 10,
  };

  async function createTokenGatedRoom() {
    const address = getAddress(appProperties.token);
    console.log(address);
    const token = await fetchToken({ address, chainId: 80001 });
    const tokenGatedRoom: TokenGatedRoom = {
      name: appProperties.name,
      signer: toBytes(appProperties.signer),
      token: toBytes(appProperties.token),
      threshold: numberToBytes(
        parseUnits(appProperties.threshold.toString(), token.decimals)
      ),
    };
    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: "lobby",
        zome_name: "lobby",
        fn_name: "create_token_gated_room",
        payload: tokenGatedRoom,
      });
      dispatch("room-created", { postHash: record.signed_action.hashed.hash });
    } catch (e) {
      console.log(e);
      // errorSnackbar.labelText = `Error creating the post: ${e.data.data}`;
      // errorSnackbar.show();
    }
  }
</script>

<div
  class="flex flex-col gap-y-4 bg-white rounded-xl border p-4 w-full max-w-md"
>
  <Heading tag="h4">Create a token gated room</Heading>
  <span>Room name</span>
  <Input type="text" bind:value={appProperties.name} />
  <span>Trusted proof signer</span>
  <Input type="text" bind:value={appProperties.signer} />
  <span>Token address</span>
  <Input type="text" bind:value={appProperties.token} />
  <span>Token threshold</span>
  <Input type="number" bind:value={appProperties.threshold} />
  <Button on:click={() => createTokenGatedRoom()}>Create</Button>
</div>
