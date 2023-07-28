import { writable } from 'svelte/store';
import type { Settings } from '$lib/types/types';

const SettingStore = writable<Settings>({
	length: 16,
	upper: false,
	number: false,
	symbol: false
});

export default SettingStore;
