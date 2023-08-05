import type { CellId } from "@holochain/client";
import { writable } from "svelte/store";

export const clientContext = 'appAgentClient';
export const currentGatedDnaProperties = 'currentGatedDnaProperties';

export const currentGatedClone = writable(null as CellId | null);
