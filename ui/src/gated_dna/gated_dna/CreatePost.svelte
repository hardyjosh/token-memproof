<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Post } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let message: string = '';

let errorSnackbar: Snackbar;

$: message;
$: isPostValid = true && message !== '';

onMount(() => {
});

async function createPost() {  
  const postEntry: Post = { 
    message: message!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'gated_dna',
      zome_name: 'gated_dna',
      fn_name: 'create_post',
      payload: postEntry,
    });
    dispatch('post-created', { postHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the post: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Post</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Message" value={ message } on:input={e => { message = e.target.value; } } required></mwc-textfield>          
  </div>
            

  <mwc-button 
    raised
    label="Create Post"
    disabled={!isPostValid}
    on:click={() => createPost()}
  ></mwc-button>
</div>
