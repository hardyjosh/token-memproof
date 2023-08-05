<script lang="ts">
  import { onMount, getContext } from "svelte";
  import type {
    EntryHash,
    Record,
    AgentPubKey,
    ActionHash,
    AppAgentClient,
    NewEntryAction,
    CellId,
    RoleName,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import PostDetail from "./PostDetail.svelte";
  import type { GatedDnaSignal } from "./types";
  import { Spinner } from "flowbite-svelte";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  export let cellId: CellId;
  let cloneId: RoleName;

  let hashes: Array<ActionHash> | undefined;
  let loading = true;
  let error: any = undefined;

  $: hashes, loading, error;

  onMount(async () => {
    await fetchPosts();
    client.on("signal", (signal) => {
      if (signal.zome_name !== "gated_dna") return;
      const payload = signal.payload as GatedDnaSignal;
      if (payload.type !== "EntryCreated") return;
      if (payload.app_entry.type !== "Post") return;
      hashes = [...hashes, payload.action.hashed.hash];
    });
  });

  async function fetchPosts() {
    // console.log(cellId);
    try {
      const records = await client.callZome({
        cell_id: cellId,
        cap_secret: null,
        zome_name: "gated_dna",
        fn_name: "get_all_posts",
        payload: null,
      });
      // console.log(cellId, records);
      hashes = records.map((r) => r.signed_action.hashed.hash);
    } catch (e) {
      error = e;
    }
    loading = false;
  }
</script>

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <Spinner />
  </div>
{:else if error}
  <span>Error fetching the posts: {error?.data?.data || error}.</span>
{:else if hashes.length === 0}
  <span>No posts found.</span>
{:else}
  <div style="display: flex; flex-direction: column">
    {#each hashes as hash}
      <div style="margin-bottom: 8px;">
        <PostDetail
          {cellId}
          postHash={hash}
          on:post-deleted={() => fetchPosts()}
        />
      </div>
    {/each}
  </div>
{/if}
