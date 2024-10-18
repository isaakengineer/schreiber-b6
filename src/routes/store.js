import { writable } from 'svelte/store';

const anzahlAenderungen = writable(0);
const Ausstattung = writable({
	haupt: "nichts",
	identitaet: {
		name: '',
		endung: '',
		dateipfad: '',
	},
	editorStatus: "default", // 'schreiben', 'pause', 'error'
})

export { anzahlAenderungen, Ausstattung };
