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
  import { Button, Spinner } from "flowbite-svelte";
  import CreateTokenGatedRoom from "./CreateTokenGatedRoom.svelte";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let hashes: Array<ActionHash> | undefined;
  let loading = true;
  let error: any = undefined;

  let showCreateTokenGatedRoom = false;

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
{:else}
  <div class="w-screen flex justify-between p-4 pb-0">
    <span>{hashes?.length} rooms found</span>
    <Button
      on:click={() => {
        showCreateTokenGatedRoom = true;
      }}>Create Room</Button
    >
  </div>
  <div class="grid grid-cols-3 gap-8 p-4">
    {#each hashes as hash}
      <div class="border rounded-xl p-4">
        <TokenGatedRoomDetail
          tokenGatedRoomHash={hash}
          on:token-gated-room-deleted={() => fetchTokenGatedRooms()}
        />
      </div>
    {/each}
  </div>
{/if}
{#if showCreateTokenGatedRoom}
  <div
    class="fixed inset-0 w-screen h-screen flex flex-col justify-center items-center z-100 bg-gray-800 bg-opacity-70"
  >
    <CreateTokenGatedRoom
      on:room-created={() => {
        showCreateTokenGatedRoom = false;
      }}
    />
  </div>
{/if}
