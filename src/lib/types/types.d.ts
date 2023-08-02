export type Item = {
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

export type Settings = {
	length: number;
	upper: boolean;
	number: boolean;
	symbol: boolean;
};

export type Client = {
	id: number;
	username: string;
	items: Item[];
};

export type TauriResponse = {
	data: any;
	status: number;
};
