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
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { TokenGatedRoom } from "./types";
  import { toHex } from "viem";
  import { fetchToken } from "@wagmi/core";
  import { account, walletClient } from "svelte-wagmi-stores";
  import { Button } from "flowbite-svelte";
  import CreatePost from "../../gated_dna/gated_dna/CreatePost.svelte";

  const dispatch = createEventDispatcher();

  export let tokenGatedRoomHash: ActionHash;

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let loading = true;
  let error: any = undefined;

  let cellId: [DnaHash, AgentPubKey];

  let record: Record | undefined;
  let tokenGatedRoom: TokenGatedRoom | undefined;

  interface TokenGatedRoomDisplay {
    name: string;
    token: string;
    threshold: number;
    signer: string;
  }
  let tokenGatedRoomDisplay: TokenGatedRoomDisplay | undefined;

  $: error, loading, record, tokenGatedRoom;

  onMount(async () => {
    if (tokenGatedRoomHash === undefined) {
      throw new Error(
        `The tokenGatedRoomHash input is required for the TokenGatedRoomDetail element`
      );
    }
    await fetchTokenGatedRoom();
  });

  async function fetchTokenGatedRoom() {
    loading = true;
    error = undefined;
    record = undefined;
    tokenGatedRoom = undefined;

    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: "lobby",
        zome_name: "lobby",
        fn_name: "get_token_gated_room",
        payload: tokenGatedRoomHash,
      });
      if (record) {
        tokenGatedRoom = decode(
          (record.entry as any).Present.entry
        ) as TokenGatedRoom;
        const token = await fetchToken({
          address: toHex(tokenGatedRoom.token),
        });
        // console.log(token);
        tokenGatedRoomDisplay = {
          name: tokenGatedRoom.name,
          token: token.name,
          threshold: tokenGatedRoom.threshold,
          signer: toHex(tokenGatedRoom.signer),
        };
        // console.log(tokenGatedRoomDisplay);
      }
    } catch (e) {
      console.log(e);
      error = e;
    }

    loading = false;
  }

  const fetchTokenBalanceSig = async () => {
    const resp = await fetch(
      `http://209.97.181.57/${toHex(tokenGatedRoom.token)}/${$account.address}`
    );
    const data = await resp.json();
    console.log(data);
    return data;
  };

  const createClone = async () => {
    const balanceProof = await fetchTokenBalanceSig();
    // const signature = await $walletClient.signMessage({
    //   account: $account.address,
    //   message: { raw: toHex(client.myPubKey) },
    // });
    const clone = await client
      .createCloneCell({
        role_name: "gated_dna",
        modifiers: {
          network_seed: "test",
          properties: tokenGatedRoomDisplay,
        },
        membrane_proof: new Uint8Array([1, 1, 1]),
      })
      .catch((e) => console.log(e));
    console.log(clone);
    cellId = clone.cell_id;
    const decoded = decode(
      clone.dna_modifiers.properties
    ) as TokenGatedRoomDisplay;
    console.log(decoded);
  };

  $: console.log($account);
  $: console.log(cellId);
</script>

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate />
  </div>
{:else if error}
  <span>Error fetching the token gated room: {error.data.data}</span>
{:else}
  <div class="flex flex-col">
    <div class="flex flex-col">
      <span><strong>Name:</strong></span>
      <span>{tokenGatedRoomDisplay.name}</span>
    </div>
    <div class="flex flex-col">
      <span><strong>Token:</strong></span>
      <span>{tokenGatedRoomDisplay.token}</span>
    </div>
    <div class="flex flex-col">
      <span><strong>Signer:</strong></span>
      <span>{tokenGatedRoomDisplay.signer}</span>
    </div>
    <div class="flex flex-col">
      <span><strong>Threshold:</strong></span>
      <span>{tokenGatedRoomDisplay.threshold}</span>
    </div>
    <Button on:click={createClone}>Join room</Button>
    {#if cellId}
      <CreatePost {cellId} />
    {/if}
  </div>
{/if}
