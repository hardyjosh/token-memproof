<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import type {
    AppAgentClient,
    Record,
    EntryHash,
    AgentPubKey,
    ActionHash,
    DnaHash,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { Post } from "./types";
  import { Button, Input } from "flowbite-svelte";

  export let cellId: [DnaHash, AgentPubKey];

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let message: string = "";

  let errorSnackbar;

  $: message;
  $: isPostValid = true && message !== "";

  onMount(() => {});

  async function createPost() {
    const postEntry: Post = {
      message: message!,
    };

    try {
      const record: Record = await client.callZome({
        cell_id: cellId,
        cap_secret: null,
        role_name: "gated_dna",
        zome_name: "gated_dna",
        fn_name: "create_post",
        payload: postEntry,
      });
      dispatch("post-created", { postHash: record.signed_action.hashed.hash });
    } catch (e) {
      errorSnackbar.labelText = `Error creating the post: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
  $: console.log(message);
</script>

<div class="flex flex-col gap-y-4">
  <span class="text-lg">Create Post</span>

  <Input type="text" bind:value={message} required />

  <Button disabled={!isPostValid} on:click={() => createPost()}>
    Create Post</Button
  >
</div>
