<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import type {
    AppAgentClient,
    Record,
    EntryHash,
    AgentPubKey,
    DnaHash,
    ActionHash,
  } from "@holochain/client";
  import { decode } from "@msgpack/msgpack";
  import { clientContext } from "../../contexts";
  import type { Post } from "./types";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  export let originalPostHash!: ActionHash;

  export let currentRecord!: Record;
  let currentPost: Post = decode(
    (currentRecord.entry as any).Present.entry
  ) as Post;

  let message: string | undefined = currentPost.message;

  let errorSnackbar: Snackbar;

  $: message;
  $: isPostValid = true && message !== "";

  onMount(() => {
    if (currentRecord === undefined) {
      throw new Error(
        `The currentRecord input is required for the EditPost element`
      );
    }
    if (originalPostHash === undefined) {
      throw new Error(
        `The originalPostHash input is required for the EditPost element`
      );
    }
  });

  async function updatePost() {
    const post: Post = {
      message: message!,
    };

    try {
      const updateRecord: Record = await client.callZome({
        cap_secret: null,
        role_name: "gated_dna",
        zome_name: "gated_dna",
        fn_name: "update_post",
        payload: {
          original_post_hash: originalPostHash,
          previous_post_hash: currentRecord.signed_action.hashed.hash,
          updated_post: post,
        },
      });

      dispatch("post-updated", {
        actionHash: updateRecord.signed_action.hashed.hash,
      });
    } catch (e) {
      errorSnackbar.labelText = `Error updating the post: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Post</span>

  <div style="margin-bottom: 16px">
    <mwc-textfield
      outlined
      label="Message"
      value={message}
      on:input={(e) => {
        message = e.target.value;
      }}
      required
    />
  </div>

  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch("edit-canceled")}
      style="flex: 1; margin-right: 16px"
    />
    <mwc-button
      raised
      label="Save"
      disabled={!isPostValid}
      on:click={() => updatePost()}
      style="flex: 1;"
    />
  </div>
</div>
