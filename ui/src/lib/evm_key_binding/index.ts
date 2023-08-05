import type { AppAgentClient } from "@holochain/client";
import type { EvmKeyBinding } from "../../lobby/lobby/types";


export const getEvmKeyBinding = async (client: AppAgentClient): Promise<{ evmKeyBinding: EvmKeyBinding, error }> => {
    let evmKeyBinding: EvmKeyBinding | null
    let error

    try {
        const evmKeyBinding = await client.callZome({
            cap_secret: null,
            role_name: "lobby",
            zome_name: "lobby",
            fn_name: "get_evm_key_binding",
            payload: null,
        });
        return { evmKeyBinding, error };
    } catch (error) {
        console.log(error?.data?.data || error);
        return { evmKeyBinding, error }
    }
}