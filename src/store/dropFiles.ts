import { writable } from 'svelte/store';


export const dropFileVisible = writable(null);
export const dropFileFiles = writable([]);
export const dropFileLoading = writable(null);
export const dropFileProgressList = writable([]);