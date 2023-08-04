import { writable } from 'svelte/store';

const SettingStore = writable<Settings>({
	length: 16,
	upper: false,
	number: false,
	symbol: false
});

export default SettingStore;
