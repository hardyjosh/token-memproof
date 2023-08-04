<script lang="ts">
  import type { TokenGatedRoom } from "./../../lobby/lobby/types.ts";
  import { Button, Heading, Input } from "flowbite-svelte";
  import { clientContext } from "../../contexts";
  import type { AppAgentClient, Record } from "@holochain/client";
  import { getContext } from "svelte";
  import { decode, encode } from "@msgpack/msgpack";
  import { toBytes } from "viem";
  import type { AppProperties } from "./types.js";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let appProperties: AppProperties = {
    name: "",
    signer: "",
    token: "",
    threshold: 0,
  };

  async function createTokenGatedRoom() {
    const tokenGatedRoom: TokenGatedRoom = {
      name: appProperties.name,
      signer: toBytes(appProperties.signer),
      token: toBytes(appProperties.token),
      threshold: Number(appProperties.threshold),
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
