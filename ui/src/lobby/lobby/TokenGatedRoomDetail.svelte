<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from "svelte";
  import { decode, encode } from "@msgpack/msgpack";
  import {
    type Record,
    type ActionHash,
    type AppAgentClient,
    type EntryHash,
    type AgentPubKey,
    type DnaHash,
    encodeHashToBase64,
    type RoleName,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { MemProof, TokenGatedRoom } from "./types";
  import { bytesToBigint, formatEther, formatUnits, toHex } from "viem";
  import { fetchToken } from "@wagmi/core";
  import { account, walletClient } from "svelte-wagmi-stores";
  import { Button } from "flowbite-svelte";
  import CreatePost from "../../gated_dna/gated_dna/CreatePost.svelte";
  import { getEvmKeyBinding } from "../../lib/evm_key_binding";
  import { fetchTokenProof } from "../../lib/fetch_proof";
  import AllPosts from "../../gated_dna/gated_dna/AllPosts.svelte";
  import { areUint8ArraysEqual } from "../../lib/utils";

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
    const matchingClone = (await client.appInfo()).cell_info.gated_dna.find(
      (cell) => {
        if (!cell?.cloned?.dna_modifiers?.properties) return false;
        return areUint8ArraysEqual(
          cell?.cloned?.dna_modifiers?.properties,
          encodedDnaProperties
        );
      }
    )?.cloned;
    // console.log({ matchingClone });
    if (matchingClone) {
      cellId = matchingClone.cell_id;
      cloneId = matchingClone.clone_id;
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

  const createClone = async () => {
    const evmKeyBinding = await getEvmKeyBinding(client);
    // console.log(evmKeyBinding);
    const tokenProof = await fetchTokenProof(
      80001,
      tokenGatedRoom.token,
      $account.address
    );
    const membrane_proof: MemProof = {
      evm_key_binding: evmKeyBinding.evmKeyBinding,
      token_proof: tokenProof,
    };
    // console.log(tokenProof);
    const clone = await client
      .createCloneCell({
        role_name: "gated_dna",
        modifiers: {
          network_seed: "test",
          properties: dnaProperties,
        },
        membrane_proof: encode(membrane_proof),
      })
      .catch((e) => console.log(e));
    console.log(clone);
    cellId = clone?.cell_id;
    cloneId = clone?.clone_id;
    const decoded = decode(
      clone.dna_modifiers.properties
    ) as TokenGatedRoomDisplay;
    // console.log(decoded);
  };

  // $: console.log({
  //   dna: encodeHashToBase64(cellId?.[0] || new Uint8Array(0)),
  //   agentKey: encodeHashToBase64(cellId?.[1] || new Uint8Array(0)),
  // });
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
      <AllPosts {cellId} />
    {/if}
  </div>
{/if}
