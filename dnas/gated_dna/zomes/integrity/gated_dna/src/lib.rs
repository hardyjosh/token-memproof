pub mod post;
use std::sync::Arc;
use ethers_core::types::{H160, RecoveryMessage, U256};
pub use post::*;
use hdi::prelude::*;
#[hdk_entry_helper]
pub struct AppProperties {
    pub name: String,
    pub signer: String,
    pub token: String,
    pub threshold: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ByteArray(#[serde(with = "serde_bytes")] Vec<u8>);
impl ByteArray {
    pub fn new(vec: Vec<u8>) -> Self {
        ByteArray(vec)
    }
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub struct EvmKeyBinding {
    pub evm_key: ByteArray,
    pub signature_bytes: ByteArray,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub struct TokenProof {
    token: ByteArray,
    owner: ByteArray,
    signer_address: ByteArray,
    balance: ByteArray,
    block: ByteArray,
    message: ByteArray,
    signature: ByteArray,
  }
#[hdk_entry_helper]
pub struct MemProof {
    evm_key_binding: EvmKeyBinding,
    token_proof: TokenProof,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Post(Post),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    PostUpdates,
    AllPosts,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    let try_properties: Result<AppProperties, _> =_data.dna_info.properties.try_into();
    let properties: AppProperties = match try_properties {
        Ok(properties) => {
            properties
        }
        Err(_) => {return Ok(ValidateCallbackResult::Valid)}
    };
    debug!("dna_info: {:?}", properties);
    // we must have some properties now, which means we also need a membrane proof
    let membrane_proof = match _data.membrane_proof {
        Some(proof) => {
            // we have some membrane proof
            let mem_proof = Arc::<hdi::prelude::SerializedBytes>::try_unwrap(proof).ok().unwrap();
            let try_membrane_proof: Result<MemProof, _> = mem_proof.try_into();
            match try_membrane_proof {
                Ok(r) => r,
                Err(_) => {return Ok(ValidateCallbackResult::Valid)}
            }
        }
        // we have properties but no membrane proof
        None => return Ok(ValidateCallbackResult::Invalid(String::from("No membrane proof")))
    };
    debug!("membrane_proof: {:?}", membrane_proof);
    // validate the evm key binding
    let mut address_array = [0u8; 20];
    address_array.copy_from_slice(membrane_proof.evm_key_binding.evm_key.into_vec().as_slice());
    let address = H160::from(address_array);
    let signature: ethers_core::types::Signature = membrane_proof.evm_key_binding.signature_bytes.into_vec().as_slice().try_into().unwrap();
    let message: RecoveryMessage = _data.agent_key.get_raw_39().try_into().ok().unwrap();
    let verified = signature.verify(message, address);
    if !verified.is_ok() {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("EVM pubkey binding signature is invalid"),
            ),
        )
    }    

    // ensure the signer of the token proof is the same signer in the app properties
    let properties_signer: Vec<u8> = hex::decode(properties.signer.strip_prefix("0x").unwrap_or(&properties.signer)).unwrap();

    if !membrane_proof.token_proof.signer_address.clone().into_vec().as_slice().eq(
        properties_signer.as_slice()
    ) {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Token proof signer address does not match app properties signer address"),
            ),
        )
    }

    // concatenate the token, owner, balance and block into one [u8]

    // verify that the message is the same as the concatenated [u8]

    // verify the signature of the message against the signer of the proof
    let proof_signer: H160 = H160::from_slice(membrane_proof.token_proof.signer_address.clone().into_vec().as_slice());
    let signature: ethers_core::types::Signature = membrane_proof.token_proof.signature.into_vec().as_slice().try_into().unwrap();
    let message = membrane_proof.token_proof.message.into_vec();
    let hash = hash_keccak256(message).ok().unwrap();
    let verified = signature.verify(hash, proof_signer);
    if !verified.is_ok() {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Token proof signature is invalid"),
            ),
        )
    }    

    // ensure that the evm key in the evm key binding is the same as the owner key in the token proof
    let proof_owner: H160 = H160::from_slice(membrane_proof.token_proof.owner.clone().into_vec().as_slice());
    if !proof_owner.eq(&address) {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Token proof owner does not match evm key binding evm key"),
            ),
        )
    }

    // ensure that the user's balance in the token proof is greater than the threshold in the dna properties
    let proof_balance: U256 = U256::from_big_endian(membrane_proof.token_proof.balance.clone().into_vec().as_slice());
    let dna_threshold: Vec<u8> = hex::decode(properties.threshold.strip_prefix("0x").unwrap_or(&properties.threshold)).unwrap();
    let dna_threshold: U256 = U256::from_big_endian(dna_threshold.as_slice());
    if proof_balance < dna_threshold {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Token proof balance is less than dna threshold. Threshold: ") + &dna_threshold.to_string() + &String::from(" Balance: ") + &proof_balance.to_string()
            ),
        )
    }

    // otherwise ok
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    return Ok(ValidateCallbackResult::Valid);
    // debug!("Hello from debug!");
    // println!("Hello from println!");
    // let dna_info: Result<AppProperties, _> = dna_info().ok().unwrap().properties.try_into();
    // match dna_info {
    //     Ok(dna_info) => {
    //         info!("dna_info {:?}", dna_info);
    //         // implement the check against the membrane proof here
    //         debug!("dna_info: {:?}", dna_info);
    //         Ok(ValidateCallbackResult::Valid)
    //     }
    //     Err(_) => Ok(ValidateCallbackResult::Invalid(String::from("didn't work")))
    // }
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Post(post) => {
                            validate_create_post(
                                EntryCreationAction::Create(action),
                                post,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Post(post) => {
                            validate_create_post(
                                EntryCreationAction::Update(action),
                                post,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    original_action,
                    original_app_entry,
                    app_entry,
                    action,
                } => {
                    match (app_entry, original_app_entry) {
                        (EntryTypes::Post(post), EntryTypes::Post(original_post)) => {
                            validate_update_post(
                                action,
                                post,
                                original_action,
                                original_post,
                            )
                        }
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original and updated entry types must be the same"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterDelete(delete_entry) => {
            match delete_entry {
                OpDelete::Entry { original_action, original_app_entry, action } => {
                    match original_app_entry {
                        EntryTypes::Post(post) => {
                            validate_delete_post(action, original_action, post)
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
            match link_type {
                LinkTypes::PostUpdates => {
                    validate_create_link_post_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPosts => {
                    validate_create_link_all_posts(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
            match link_type {
                LinkTypes::PostUpdates => {
                    validate_delete_link_post_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPosts => {
                    validate_delete_link_all_posts(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Post(post) => {
                            validate_create_post(
                                EntryCreationAction::Create(action),
                                post,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::Post(post) => {
                            let result = validate_create_post(
                                EntryCreationAction::Update(action.clone()),
                                post.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_post: Option<Post> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_post = match original_post {
                                    Some(post) => post,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_post(
                                    action,
                                    post,
                                    original_action,
                                    original_post,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            if original_action.entry_type().visibility().is_public() {
                                return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                            } else {
                                return Ok(ValidateCallbackResult::Valid);
                            }
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index.clone(),
                        app_entry_type.entry_index.clone(),
                        &entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::Post(original_post) => {
                            validate_delete_post(action, original_action, original_post)
                        }
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    match link_type {
                        LinkTypes::PostUpdates => {
                            validate_create_link_post_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllPosts => {
                            validate_create_link_all_posts(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index.clone(),
                        create_link.link_type.clone(),
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::PostUpdates => {
                            validate_delete_link_post_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllPosts => {
                            validate_delete_link_all_posts(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
