import { writable } from 'svelte/store';
import type { Client } from '$lib/types/types';

const ClientStore = writable<Client>();

export default ClientStore;
