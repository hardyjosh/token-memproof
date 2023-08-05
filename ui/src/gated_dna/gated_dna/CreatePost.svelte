<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import {
    type AppAgentClient,
    type Record,
    type EntryHash,
    type AgentPubKey,
    type ActionHash,
    type DnaHash,
    type RoleName,
    encodeHashToBase64,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { Post } from "./types";
  import { Button, Input } from "flowbite-svelte";
  import { addSnackBar } from "../../lib/snackbar/snackbar";

  export let cellId: [DnaHash, AgentPubKey];

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let message: string = "";

  $: message;
  $: isPostValid = true && message !== "";

  onMount(() => {});

  async function createPost() {
    const postEntry: Post = {
      message: message!,
    };

    try {
      console.log("cell id whilst creating post", {
        dna: encodeHashToBase64(cellId?.[0] || new Uint8Array(0)),
        agentKey: encodeHashToBase64(cellId?.[1] || new Uint8Array(0)),
      });
      const record: Record = await client.callZome({
        cell_id: cellId,
        cap_secret: null,
        zome_name: "gated_dna",
        fn_name: "create_post",
        payload: postEntry,
      });
      dispatch("post-created", { postHash: record.signed_action.hashed.hash });
      message = "";
    } catch (e) {
      addSnackBar(`Error creating the evm key binding: ${e?.data?.data || e}`);
    }
  }
</script>

<div class="flex gap-x-4">
  <Input type="text" bind:value={message} required />

  <div class="flex-shrink-0">
    <Button disabled={!isPostValid} on:click={() => createPost()}>
      Create Post</Button
    >
  </div>
</div>
