import { invoke } from '@tauri-apps/api';
import type { Writable } from 'svelte/store';

export const generatePassword = async (settings: Settings): Promise<string> => {
	return await invoke('generate_password', {
		length: settings.length,
		upper: settings.upper,
		numbers: settings.number,
		symbols: settings.symbol
	});
};

export function localToStore(store: Writable<Settings | Client>, key: string, initial: any) {
	let result = JSON.parse(localStorage.getItem(key) || 'null');
	if (result !== null) {
		store.set(result);
	} else {
		store.set(initial);
	}
}
