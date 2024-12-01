import wasm_init, { initSync } from "../build/decaf_lib.js";

export const init = wasm_init;
export const init_sync = initSync;
export * from "../build/decaf_lib.js";
