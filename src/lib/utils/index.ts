import { invoke } from '@tauri-apps/api';

export const generatePassword = async (
	length: number,
	upper: boolean,
	numbers: boolean,
	symbols: boolean
): Promise<string> => {
	return await invoke('generate_password', {
		length,
		upper,
		numbers,
		symbols
	});
};
