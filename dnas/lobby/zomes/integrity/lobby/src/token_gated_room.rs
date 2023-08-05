use hdi::prelude::*;
use crate::ByteArray;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct TokenGatedRoom {
    pub name: String,
    pub token: ByteArray,
    pub signer: ByteArray,
    pub threshold: ByteArray,
}
pub fn validate_create_token_gated_room(
    _action: EntryCreationAction,
    _token_gated_room: TokenGatedRoom,
) -> ExternResult<ValidateCallbackResult> {
    if *_action.action_seq() < 5u32 {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("EVM pubkey binding must be the first action after genesis"),
            ),
        )
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_token_gated_room(
    _action: Update,
    _token_gated_room: TokenGatedRoom,
    _original_action: EntryCreationAction,
    _original_token_gated_room: TokenGatedRoom,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Token Gated Rooms cannot be updated"),
        ),
    )
}
pub fn validate_delete_token_gated_room(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_token_gated_room: TokenGatedRoom,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Token Gated Rooms cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_lobbies(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _token_gated_room: crate::TokenGatedRoom = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_lobbies(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllLobbies links cannot be deleted"),
        ),
    )
}
