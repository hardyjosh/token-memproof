use hdk::prelude::*;
use lobby_integrity::*;
#[hdk_extern]
pub fn create_token_gated_room(
    token_gated_room: TokenGatedRoom,
) -> ExternResult<Record> {
    let token_gated_room_hash = create_entry(
        &EntryTypes::TokenGatedRoom(token_gated_room.clone()),
    )?;
    let record = get(token_gated_room_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created TokenGatedRoom"))
            ),
        )?;
    let path = Path::from("all_lobbies");
    create_link(
        path.path_entry_hash()?,
        token_gated_room_hash.clone(),
        LinkTypes::AllLobbies,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_token_gated_room(
    token_gated_room_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(token_gated_room_hash, GetOptions::default())
}
