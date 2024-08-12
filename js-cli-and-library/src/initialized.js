import { initSync, build } from "./index.js";
import { readFileSync } from "node:fs";

const wasmPath = new URL("./shared/decaf_lib_bg.wasm", import.meta.url);
initSync(readFileSync(wasmPath));

export { build }
