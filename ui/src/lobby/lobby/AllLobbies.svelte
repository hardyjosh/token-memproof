<script lang="ts">
  import { onMount, getContext } from "svelte";
  import type {
    EntryHash,
    Record,
    AgentPubKey,
    ActionHash,
    AppAgentClient,
    NewEntryAction,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import TokenGatedRoomDetail from "./TokenGatedRoomDetail.svelte";
  import type { LobbySignal } from "./types";
  import { Spinner } from "flowbite-svelte";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let hashes: Array<ActionHash> | undefined;
  let loading = true;
  let error: any = undefined;

  $: hashes, loading, error;

  onMount(async () => {
    await fetchTokenGatedRooms();
    client.on("signal", (signal) => {
      if (signal.zome_name !== "lobby") return;
      const payload = signal.payload as LobbySignal;
      if (payload.type !== "EntryCreated") return;
      if (payload.app_entry.type !== "TokenGatedRoom") return;
      hashes = [...hashes, payload.action.hashed.hash];
    });
  });

  async function fetchTokenGatedRooms() {
    try {
      const records = await client.callZome({
        cap_secret: null,
        role_name: "lobby",
        zome_name: "lobby",
        fn_name: "get_all_lobbies",
        payload: null,
      });
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
  <span>Error fetching the token gated rooms: {error?.data?.data}.</span>
{:else if hashes.length === 0}
  <span>No token gated rooms found.</span>
{:else}
  <div style="display: flex; flex-direction: column">
    {#each hashes as hash}
      <div style="margin-bottom: 8px;">
        <TokenGatedRoomDetail
          tokenGatedRoomHash={hash}
          on:token-gated-room-deleted={() => fetchTokenGatedRooms()}
        />
      </div>
    {/each}
  </div>
{/if}
