import { invoke } from '@tauri-apps/api';
import type { Settings } from '$lib/types/types';
import type { Writable } from 'svelte/store';
export const generatePassword = async (settings: Settings): Promise<string> => {
	return await invoke('generate_password', {
		length: settings.length,
		upper: settings.upper,
		numbers: settings.number,
		symbols: settings.symbol
	});
};

export function localToStore(store: Writable<Settings>, key: string) {
	let result = JSON.parse(localStorage.getItem(key) || 'null');
	if (result !== null) {
		store.set(result);
	} else {
		console.log('default');
		store.set({
			length: 16,
			upper: false,
			number: false,
			symbol: false
		});
	}
}
