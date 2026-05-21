/* tslint:disable */
/* eslint-disable */

export class GridResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    data_len(): number;
    data_ptr(): number;
    size: number;
}

export class Position {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    x: number;
    y: number;
}

export function generate(size: number, offsets_a: Int32Array, offsets_b: Int32Array): GridResult;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_get_gridresult_size: (a: number) => number;
    readonly __wbg_get_position_x: (a: number) => number;
    readonly __wbg_get_position_y: (a: number) => number;
    readonly __wbg_gridresult_free: (a: number, b: number) => void;
    readonly __wbg_position_free: (a: number, b: number) => void;
    readonly __wbg_set_gridresult_size: (a: number, b: number) => void;
    readonly __wbg_set_position_x: (a: number, b: number) => void;
    readonly __wbg_set_position_y: (a: number, b: number) => void;
    readonly generate: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly gridresult_data_len: (a: number) => number;
    readonly gridresult_data_ptr: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
