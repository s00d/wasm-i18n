/* tslint:disable */
/* eslint-disable */
export function set_translations(locale: string, json: string): void;
export function set_translations_from_object(locale: string, obj: any): void;
export function get_translations(locale: string): string;
export function del_translations(locale: string): void;
export function del_translation(locale: string, key: string): void;
export function get_translation(locale: string, key: string): string;
export function has_translation(locale: string, key: string): boolean;
export function has_locale(locale: string): boolean;
export function format_translation(locale: string, key: string, args: any): string;
export function load_translations(url: string): Promise<void>;
export function err(e: string): Promise<void>;
export function get_all_locales(): any;
export function get_all_translations_for_locale(locale: string): any;
export function clear_all_translations(): void;
export function update_translation(locale: string, key: string, value: any): void;
export function get_all_translations(): any;
export function has_key_in_translations(locale: string, key: string): boolean;
