<script lang="ts">
  import { currentGatedClone } from "./../../contexts.ts";
  import { createEventDispatcher, onMount, getContext } from "svelte";
  import { decode, encode } from "@msgpack/msgpack";
  import type {
    Record,
    ActionHash,
    AppAgentClient,
    AgentPubKey,
    DnaHash,
    RoleName,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { MemProof, TokenGatedRoom } from "./types";
  import { bytesToBigint, formatEther, toHex } from "viem";
  import { fetchBalance, fetchToken } from "@wagmi/core";
  import { account } from "svelte-wagmi-stores";
  import { Button } from "flowbite-svelte";
  import CreatePost from "../../gated_dna/gated_dna/CreatePost.svelte";
  import { getEvmKeyBinding } from "../../lib/evm_key_binding";
  import { fetchTokenProof } from "../../lib/fetch_proof";
  import AllPosts from "../../gated_dna/gated_dna/AllPosts.svelte";
  import { areUint8ArraysEqual } from "../../lib/utils";
  import { addSnackBar } from "../../lib/snackbar/snackbar.js";

  const dispatch = createEventDispatcher();

  export let tokenGatedRoomHash: ActionHash;

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let loading = true;
  let error: any = undefined;

  let cellId: [DnaHash, AgentPubKey];
  let cloneId: RoleName;

  let record: Record | undefined;
  let tokenGatedRoom: TokenGatedRoom | undefined;
  let dnaProperties: any;
  let encodedDnaProperties: Uint8Array | undefined;

  interface TokenGatedRoomDisplay {
    name: string;
    token: string;
    threshold: number;
    signer: string;
  }
  let tokenGatedRoomDisplay: TokenGatedRoomDisplay | undefined;

  let hasEnoughTokens = false;
  let balanceVal: bigint | undefined;

  $: error, loading, record, tokenGatedRoom;

  onMount(async () => {
    if (tokenGatedRoomHash === undefined) {
      throw new Error(
        `The tokenGatedRoomHash input is required for the TokenGatedRoomDetail element`
      );
    }
    await fetchTokenGatedRoom();
    dnaProperties = {
      ...tokenGatedRoom,
      token: toHex(tokenGatedRoom.token),
      signer: toHex(tokenGatedRoom.signer),
      threshold: toHex(tokenGatedRoom.threshold),
    };
    encodedDnaProperties = encode(dnaProperties);
    // console.log((await client.appInfo()).cell_info.gated_dna);
    const existingClone = (await client.appInfo()).cell_info.gated_dna.find(
      (cell) => {
        if (!cell?.cloned?.dna_modifiers?.properties) return false;
        return areUint8ArraysEqual(
          cell?.cloned?.dna_modifiers?.properties,
          encodedDnaProperties
        );
      }
    )?.cloned;
    // console.log({ matchingClone });
    const balance = await fetchBalance({
      address: $account.address,
      token: toHex(tokenGatedRoom.token),
    });
    balanceVal = balance.value;

    const threshold = bytesToBigint(tokenGatedRoom.threshold);
    hasEnoughTokens = balance.value >= threshold;

    if (existingClone) {
      cellId = existingClone.cell_id;
      cloneId = existingClone.clone_id;
    }
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
          threshold: Number(
            formatEther(bytesToBigint(tokenGatedRoom.threshold))
          ),
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

  const joinRoom = async () => {
    const evmKeyBinding = await getEvmKeyBinding(client);
    const tokenProof = await fetchTokenProof(
      80001,
      tokenGatedRoom.token,
      $account.address
    );
    const membrane_proof: MemProof = {
      evm_key_binding: evmKeyBinding.evmKeyBinding,
      token_proof: tokenProof,
    };
    const clone = await client
      .createCloneCell({
        role_name: "gated_dna",
        modifiers: {
          network_seed: "test",
          properties: dnaProperties,
        },
        membrane_proof: encode(membrane_proof),
      })
      .catch((e) => {
        let message = e?.message;
        if (typeof message == "string") {
          if (message.includes("threshold")) {
            addSnackBar("You don't have enough tokens to join this room");
            return;
          }
        }
        console.log(e);
      });
    cellId = clone?.cell_id;
    cloneId = clone?.clone_id;
  };
</script>

{#if loading}
  <div
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate />
  </div>
{:else if error}
  <span>Error fetching the token gated room: {error?.data?.data || error}</span>
{:else}
  <div class="flex flex-col gap-y-2 overflow-hidden">
    <div class="flex flex-col">
      <span><strong>Name:</strong></span>
      <span>{tokenGatedRoomDisplay.name}</span>
    </div>
    <div class="flex flex-col">
      <span><strong>Token:</strong></span>
      <span>{tokenGatedRoomDisplay.token}</span>
    </div>
    <div class="flex flex-col">
      <span><strong>Threshold:</strong></span>
      <span>{tokenGatedRoomDisplay.threshold}</span>
    </div>
    <div class="flex flex-col">
      <span><strong>Your balance:</strong></span>
      {#if balanceVal}
        <span>{formatEther(balanceVal)}</span>
      {/if}
    </div>
    <div class="flex flex-col">
      <span><strong>Trusted proof signer:</strong></span>
      <span>{tokenGatedRoomDisplay.signer}</span>
    </div>
    {#if cellId}
      <Button
        on:click={() => {
          $currentGatedClone = cellId;
        }}>Enter room</Button
      >
    {:else}
      <Button on:click={joinRoom}>Join room</Button>
    {/if}
  </div>
{/if}
