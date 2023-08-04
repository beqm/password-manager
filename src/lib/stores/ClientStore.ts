import { writable } from 'svelte/store';

const ClientStore = writable<Client>();

export default ClientStore;
