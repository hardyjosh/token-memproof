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



export async function sampleEvmKeyBinding(cell: CallableCell, partialEvmKeyBinding = {}) {
    return {
        ...{
	  evm_key: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  signature_bytes: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialEvmKeyBinding
    };
}

export async function createEvmKeyBinding(cell: CallableCell, evmKeyBinding = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "lobby",
      fn_name: "create_evm_key_binding",
      payload: evmKeyBinding || await sampleEvmKeyBinding(cell),
    });
}

