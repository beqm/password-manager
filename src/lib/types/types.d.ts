export type Item = {
	title: string;
	content: string;
	link?: string;
	lastUsed: string;
	lastModified: string;
};

export type Settings = {
	length: number;
	upper: boolean;
	number: boolean;
	symbol: boolean;
};
