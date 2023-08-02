/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} rom
*/
export function load_rom(rom: Uint8Array): void;
/**
* @param {Uint8Array} rom
*/
export function boot(rom: Uint8Array): void;
/**
*/
export function run_gameboy(): void;
/**
*/
export function start(): void;
/**
*/
export function draw(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly load_rom: (a: number, b: number) => void;
  readonly boot: (a: number, b: number) => void;
  readonly run_gameboy: () => void;
  readonly start: () => void;
  readonly draw: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
