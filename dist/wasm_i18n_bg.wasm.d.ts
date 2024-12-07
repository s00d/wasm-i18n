/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const set_translations: (a: number, b: number, c: number, d: number) => [number, number];
export const get_translation: (a: number, b: number, c: number, d: number) => [number, number, number];
export const del_translation: (a: number, b: number, c: number, d: number) => [number, number];
export const set_translations_from_object: (a: number, b: number, c: any) => [number, number];
export const del_translations: (a: number, b: number) => [number, number];
export const has_locale: (a: number, b: number) => number;
export const clear_all_translations: () => [number, number];
export const load_translations: (a: number, b: number) => any;
export const get_all_locales: () => [number, number, number];
export const update_translation: (a: number, b: number, c: number, d: number, e: any) => [number, number];
export const format_translation: (a: number, b: number, c: number, d: number, e: any) => [number, number, number, number];
export const get_all_translations_for_locale: (a: number, b: number) => [number, number, number];
export const get_all_translations: () => [number, number, number];
export const has_key_in_translations: (a: number, b: number, c: number, d: number) => number;
export const has_translation: (a: number, b: number, c: number, d: number) => number;
export const get_translations: (a: number, b: number) => [number, number, number];
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_exn_store: (a: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_export_4: WebAssembly.Table;
export const __wbindgen_export_5: WebAssembly.Table;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const closure47_externref_shim: (a: number, b: number, c: any) => void;
export const closure65_externref_shim: (a: number, b: number, c: any, d: any) => void;
export const __wbindgen_start: () => void;