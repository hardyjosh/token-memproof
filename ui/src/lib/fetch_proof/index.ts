import { toBytes, toHex } from "viem";
import type { TokenProof, TokenProofResponse } from "../../lobby/lobby/types";

export const fetchTokenProof = async (chain: number, token: Uint8Array, owner: string): Promise<TokenProof> => {
    const resp = await fetch(
        `http://209.97.181.57/${chain}/${toHex(token)}/${owner}`
    );
    const data = await resp.json() as TokenProofResponse;
    const tokenProof: TokenProof = { ...data, balance: toBytes(data.balance), block: toBytes(data.block) }
    return tokenProof;
};