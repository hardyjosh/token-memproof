<script lang="ts">
  import type { TokenGatedRoom } from "./../../lobby/lobby/types.ts";
  import { Button, Heading, Input } from "flowbite-svelte";
  import { clientContext } from "../../contexts";
  import type { AppAgentClient, Record } from "@holochain/client";
  import { getContext } from "svelte";
  import { decode, encode } from "@msgpack/msgpack";
  import {
    getAddress,
    numberToBytes,
    numberToHex,
    parseUnits,
    toBytes,
  } from "viem";
  import type { AppProperties } from "./types.js";
  import { fetchToken } from "@wagmi/core";
  import { parse } from "svelte/compiler";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let appProperties: AppProperties = {
    name: "hi",
    signer: "0x3cDB3d9e1B74692Bb1E3bb5fc81938151cA64b02",
    token: "0x6c6EE5e31d828De241282B9606C8e98Ea48526E2",
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
      // dispatch("post-created", { postHash: record.signed_action.hashed.hash });
    } catch (e) {
      console.log(e);
      // errorSnackbar.labelText = `Error creating the post: ${e.data.data}`;
      // errorSnackbar.show();
    }
  }
</script>

<div>
  <Heading tag="h3">Create a token gated room</Heading>
  <Input type="text" bind:value={appProperties.name} />
  <Input type="text" bind:value={appProperties.signer} />
  <Input type="text" bind:value={appProperties.token} />
  <Input type="number" bind:value={appProperties.threshold} />
  <Button on:click={() => createTokenGatedRoom()}>Create</Button>
</div>
