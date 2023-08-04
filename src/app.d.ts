// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	type Item = {
		id: number;
		title: string;
		identifier: string;
		password: string;
		description: string;
		type_: string;
		link?: string;
		created_at: number;
		last_modified: number;
	};
	type Settings = {
		length: number;
		upper: boolean;
		number: boolean;
		symbol: boolean;
	};

	type Client = {
		id: number;
		username: string;
		items: Item[];
	};

	type TauriResponse = {
		data: any;
		status: number;
	};
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}
}

export {};
