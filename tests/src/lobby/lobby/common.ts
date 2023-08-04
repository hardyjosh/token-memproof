import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleTokenGatedRoom(cell: CallableCell, partialTokenGatedRoom = {}) {
    return {
        ...{
	  token: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  signer: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  threshold: 10,
        },
        ...partialTokenGatedRoom
    };
}

export async function createTokenGatedRoom(cell: CallableCell, tokenGatedRoom = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "lobby",
      fn_name: "create_token_gated_room",
      payload: tokenGatedRoom || await sampleTokenGatedRoom(cell),
    });
}

