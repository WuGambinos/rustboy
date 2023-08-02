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
export class WebGameBoy {
  free(): void;
/**
*/
  constructor();
/**
*/
  emu_loop(): void;
/**
* @param {Uint8Array} rom
*/
  boot(rom: Uint8Array): void;
/**
* @returns {number}
*/
  get_bc(): number;
/**
*/
  run(): void;
/**
*/
  draw(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly load_rom: (a: number, b: number) => void;
  readonly boot: (a: number, b: number) => void;
  readonly __wbg_webgameboy_free: (a: number) => void;
  readonly webgameboy_new: () => number;
  readonly webgameboy_emu_loop: (a: number) => void;
  readonly webgameboy_boot: (a: number, b: number, c: number) => void;
  readonly webgameboy_get_bc: (a: number) => number;
  readonly webgameboy_run: (a: number) => void;
  readonly webgameboy_draw: (a: number) => void;
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
