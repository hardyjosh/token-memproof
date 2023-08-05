<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from "svelte";
  import { decode } from "@msgpack/msgpack";
  import type {
    Record,
    ActionHash,
    AppAgentClient,
    EntryHash,
    AgentPubKey,
    DnaHash,
    CellId,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { Post } from "./types";
  import EditPost from "./EditPost.svelte";

  const dispatch = createEventDispatcher();

  export let postHash: ActionHash;

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  export let cellId: CellId;

  let loading = true;
  let error: any = undefined;

  let record: Record | undefined;
  let post: Post | undefined;

  let editing = false;

  let errorSnackbar: Snackbar;

  $: editing, error, loading, record, post;

  onMount(async () => {
    if (postHash === undefined) {
      throw new Error(
        `The postHash input is required for the PostDetail element`
      );
    }
    await fetchPost();
  });

  async function fetchPost() {
    loading = true;
    error = undefined;
    record = undefined;
    post = undefined;

    try {
      record = await client.callZome({
        cap_secret: null,
        cell_id: cellId,
        // role_name: "gated_dna",
        zome_name: "gated_dna",
        fn_name: "get_post",
        payload: postHash,
      });
      if (record) {
        post = decode((record.entry as any).Present.entry) as Post;
        console.log(post);
      }
    } catch (e) {
      error = e;
    }

    loading = false;
  }

  async function deletePost() {
    try {
      await client.callZome({
        cap_secret: null,
        role_name: "gated_dna",
        zome_name: "gated_dna",
        fn_name: "delete_post",
        payload: postHash,
      });
      dispatch("post-deleted", { postHash: postHash });
    } catch (e: any) {
      errorSnackbar.labelText = `Error deleting the post: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate />
  </div>
{:else if error}
  <span>Error fetching the post: {error.data.data}</span>
{:else}
  <div style="display: flex; flex-direction: column">
    <div style="display: flex; flex-direction: row">
      <span style="flex: 1" />
      <mwc-icon-button
        style="margin-left: 8px"
        icon="edit"
        on:click={() => {
          editing = true;
        }}
      />
      <mwc-icon-button
        style="margin-left: 8px"
        icon="delete"
        on:click={() => deletePost()}
      />
    </div>

    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="margin-right: 4px"><strong>Message:</strong></span>
      <span style="white-space: pre-line">{post?.message}</span>
    </div>
  </div>
{/if}
