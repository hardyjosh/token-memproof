import type {
  Record,
  ActionHash,
  DnaHash,
  SignedActionHashed,
  EntryHash,
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type LobbySignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
  | ({ type: 'EvmKeyBinding'; } & EvmKeyBinding)
  | ({ type: 'TokenGatedRoom'; } & TokenGatedRoom);



export interface TokenGatedRoom {
  name: string;

  token: Uint8Array;

  signer: Uint8Array;

  threshold: Uint8Array;
}




export interface EvmKeyBinding {
  evm_key: Uint8Array;

  signature_bytes: Uint8Array;
}

export interface TokenProof {
  token: Uint8Array;
  owner: Uint8Array;
  signer_address: Uint8Array;
  balance: Uint8Array;
  block: Uint8Array;
  message: Uint8Array;
  signature: Uint8Array;
}

export interface TokenProofResponse {
  token: Uint8Array;
  owner: Uint8Array;
  signer_address: Uint8Array;
  balance: string;
  block: string;
  message: Uint8Array;
  signature: Uint8Array;
}

export interface MemProof {
  token_proof: TokenProof;
  evm_key_binding: EvmKeyBinding;
}