import { writable } from 'svelte/store';

const csvDatei = writable({});

const reset = () => {
	csvDatei.set((v) => v = {
		kopfreihe: [],
		reihen: [],
		neueReihe: [],
		fahneGeaendert: false,
	});
}

export {
	csvDatei, reset
};
