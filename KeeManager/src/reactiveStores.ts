// stores.ts
import { writable } from 'svelte/store';
import type { Entry } from './routes/Main.svelte';

export const database = writable({ dbPath: '' , masterKey: ''});
export const entriesStore = writable<Entry[]>([]);
export const selectedEntryStore = writable<Entry | null>();
export const filterStore = writable({filter : ''});