use hdi::prelude::*;
use crate::ByteArray;
use ethers_core::types::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct EvmKeyBinding {
    pub evm_key: ByteArray,
    pub signature_bytes: ByteArray,
}

pub fn validate_create_evm_key_binding(
    _action: EntryCreationAction,
    _evm_key_binding: EvmKeyBinding,
) -> ExternResult<ValidateCallbackResult> {
    let mut address_array = [0u8; 20];
    address_array.copy_from_slice(_evm_key_binding.evm_key.into_vec().as_slice());
    let address = H160::from(address_array);
    let signature: ethers_core::types::Signature = _evm_key_binding.signature_bytes.into_vec().as_slice().try_into().unwrap();

    let message: RecoveryMessage = _action.author().get_raw_39().try_into().ok().unwrap();

    let verified = signature.verify(message, address);

    if !verified.is_ok() {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("EVM pubkey binding signature is invalid"),
            ),
        )
    }

    if *_action.action_seq() != 4u32 {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("EVM pubkey binding must be the first action after genesis"),
            ),
        )
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_evm_key_binding(
    _action: Update,
    _evm_key_binding: EvmKeyBinding,
    _original_action: EntryCreationAction,
    _original_evm_key_binding: EvmKeyBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Evm Key Bindings cannot be updated"),
        ),
    )
}
pub fn validate_delete_evm_key_binding(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_evm_key_binding: EvmKeyBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Evm Key Bindings cannot be deleted"),
        ),
    )
}
