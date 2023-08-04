<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { TokenGatedRoom } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let token!: string;

export let threshold!: number;


let signer: string = '';

let errorSnackbar: Snackbar;

$: token, signer, threshold;
$: isTokenGatedRoomValid = true && signer !== '';

onMount(() => {
  if (token === undefined) {
    throw new Error(`The token input is required for the CreateTokenGatedRoom element`);
  }
  if (threshold === undefined) {
    throw new Error(`The threshold input is required for the CreateTokenGatedRoom element`);
  }
});

async function createTokenGatedRoom() {  
  const tokenGatedRoomEntry: TokenGatedRoom = { 
    token: token!,
    signer: signer!,
    threshold: threshold!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'lobby',
      zome_name: 'lobby',
      fn_name: 'create_token_gated_room',
      payload: tokenGatedRoomEntry,
    });
    dispatch('token-gated-room-created', { tokenGatedRoomHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the token gated room: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create TokenGatedRoom</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Signer" value={ signer } on:input={e => { signer = e.target.value;} } required></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create TokenGatedRoom"
    disabled={!isTokenGatedRoomValid}
    on:click={() => createTokenGatedRoom()}
  ></mwc-button>
</div>
