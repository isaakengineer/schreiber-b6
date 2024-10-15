import { writable } from 'svelte/store';

const anzahlAenderungen = writable(0);

export { anzahlAenderungen };