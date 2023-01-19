import { writable } from 'svelte/store';

export type CategoryMap = { [category: string]: string[] };

export const categoryStore = writable<CategoryMap>({});
