<script>
	import { invoke } from "@tauri-apps/api/core";
	import { emit, once, listen } from "@tauri-apps/api/event";
	import { getCurrentWindow } from '@tauri-apps/api/window';

	// CODEMIRROR LANGUAGES
	import { syntaxHighlighting, StreamLanguage } from "@codemirror/language";
	import { stex } from "@codemirror/legacy-modes/mode/stex";
	import { markdown } from "@codemirror/lang-markdown";

	import { onMount } from 'svelte'
	import CodeMirror, { basicSetup } from '$lib/Editor/CodeMirror.svelte'

	import Datei from '$lib/datei/Datei.svelte';
	import { csvDatei, reset as csvReset } from '$lib/Editor/store.js';
	import CSVUI from '$lib/Editor/CSVUI.svelte';

	import XCircle from "phosphor-svelte/lib/XCircle";

	import { anzahlAenderungen, Ausstattung } from './store.js';

	const AUTOSAVE_TRIGGER = 128;
	csvReset();
	let store;
	let initialStore;
	let elementalCloseButtonWeight = "duotone";
	const appWindow = getCurrentWindow();
	let message = "";
	let extensionsList = [];
	let pfad = [];
	let csvgeandert = false;

	onMount(async () => {
		await invoke('init_pruefen')
			.then((payload) => {
				$Ausstattung.identitaet = payload;
				init();
			}).catch(() => {
				console.log("nicht init")
			});
  		extensionsList.push(basicSetup);
		const cl = document.body.classList
		if (cl.contains('dark')) {
			cl.remove('dark')
		  return () => void cl.add('dark')
		}
	})

	const init = async () => {
		console.log("initializing")
		switch ($Ausstattung.identitaet.endung) {
			case "tex":
				extensionsList.push(StreamLanguage.define(stex));
				$Ausstattung.haupt = "text";
				initialStore = await invoke("lesen");
			break
			case "md":
				extensionsList.push(markdown());
				$Ausstattung.haupt = "text";
				initialStore = await invoke("lesen");
			break
			case "csv":
				let headers = await invoke("csv_lesen_kopf");
				let records = await invoke("csv_lesen_reihen");
				$csvDatei.kopfreihe = headers;
				$csvDatei.reihen = records;
				$csvDatei.neueReihe = Array(headers.length)
				console.log(records)
				console.log(headers)
				$Ausstattung.haupt = "datei";
			break
			default:
				$Ausstattung.haupt = "text";
				initialStore = await invoke("lesen");
			break
		}
		pfad = $Ausstattung.identitaet.dateipfad.split("/").reverse().slice(1,4);
	}

	listen("datei-gewaehlt", async (event) => {
		$Ausstattung.identitaet = event.payload;
		init();
	});

	async function changeHandler({ detail: {trs} }) {
		const changes = trs.map(tr => tr.changes)
		console.log('change', changes.map(ch => ch.toJSON()))
		if (changes.length > 1) {
			const totalChange = changes.reduce((c1, c2) => c1.compose(c2))
			console.log('totalChange', totalChange.toJSON())
		}
		$anzahlAenderungen = $anzahlAenderungen + changes.length;
		$Ausstattung.editorStatus = "schreiben";
		if ( $anzahlAenderungen > AUTOSAVE_TRIGGER )
		{
			speichern()
		}
	}

	const speichern = async () => {
		$Ausstattung.editorStatus = "pause";
		let geschrieben = await invoke("schreiben", { text: $store })
			.catch((err) => {
				console.log(err)
				message = err;
				return false;
				$Ausstattung.editorStatus = "error";
			});
		if ( geschrieben ) {
			$anzahlAenderungen = 0;
			geschrieben = undefined;
			$Ausstattung.editorStatus = "default"
		} else {

			console.error("HUGE ERROR!")
		}
	}
	const CSVSpeichern = async () => {
		$Ausstattung.editorStatus = "pause";
		console.log($csvDatei.neueReihe.length)
		let neueReihe = Array()
		for (let i = 0; i < $csvDatei.kopfreihe.length; i++) {
			neueReihe.push((!$csvDatei.neueReihe[i]) ? "" : $csvDatei.neueReihe[i]);
		}
		console.log(neueReihe)
		let geschrieben = await invoke("csv_schreiben", {reihe: neueReihe});
		if ( geschrieben ) {
			let headers = await invoke("csv_lesen_kopf");
			let records = await invoke("csv_lesen_reihen");
			$csvDatei.fahneGeaendert = false;
			$csvDatei.kopfreihe = headers;
			$csvDatei.reihen = records;
			$csvDatei.neueReihe = []
			for (let i = 0; i < $csvDatei.kopfreihe.length; i++) {
				$csvDatei.neueReihe.push("");
			}
			$Ausstattung.editorStatus = 'default';
		} else {
			console.log("something went terribly wrong")
		}
	}
</script>

<div class="haupt {$Ausstattung.haupt}">
	<header>
		{#if $Ausstattung.identitaet && $Ausstattung.identitaet.name}

			{#if $Ausstattung.haupt === "text"}
				<button
					class="{$Ausstattung.editorStatus} fahne"
					on:click={speichern}>
					{#if $anzahlAenderungen > 0}{$anzahlAenderungen}{/if}
				</button>
			{:else if $Ausstattung.haupt === "datei"}
				<button
					class="{$Ausstattung.editorStatus} fahne"
					on:click={CSVSpeichern}>
					{#if $csvDatei.fahneGeaendert }speichern{/if}
				</button>
			{/if}

			<div class="title">
				<div class="name">{ $Ausstattung.identitaet.name }</div>
				<!-- <div class="pfad">{ $Ausstattung.identitaet.dateipfad }</div> -->
				<div class="pfad">
					{#each pfad as p}<span>{p}</span>{/each}
				</div>
			</div>
		{:else}
			<div class="app-name">Schreiber-B6</div>
			<nav>
	        	<ul>
	         		<li>Einstellung</li>
	           		<li>Über</li>
	             	<li>Dokumentation</li>
	            </ul>
	        </nav>
		{/if}
	</header>
	<div class="movable">
		<button class="windowdragger" data-tauri-drag-region></button>
	</div>
	<div class="tauri">
		<button class="elemental titlebar-button" id="titlebar-close"
				on:mouseover={() => elementalCloseButtonWeight = "fill"}
				on:mouseleave={() => elementalCloseButtonWeight = "duotone"}
				on:click={() => appWindow.close()}>
			<XCircle size="1.8em" bind:weight={elementalCloseButtonWeight} />
		</button>
	</div>
	<main>
		{#if $Ausstattung.haupt === "nichts"}
			<Datei />
		{:else if $Ausstattung.haupt === "datei"}
			<CSVUI />
		{:else if $Ausstattung.haupt === "text"}
			<div class="schreiber">
				<CodeMirror
					doc={initialStore}
					bind:docStore={store}
					extensions={extensionsList}
					on:change={changeHandler}></CodeMirror>
				<div class="fake">
					<div class="line"></div>
					<div class="editor"></div>
				</div>
			</div>
		{/if}
	</main>
	<footer>
		{#if $Ausstattung.identitaet.dateipfad}
		<div class="ueber">
			<div class="sprache">{ $Ausstattung.identitaet.endung }</div>
		</div>
		{/if}
	</footer>
</div>

<style lang="scss">
:global(.codemirror) {
	// border: 1px solid #ddd;
}
.haupt {
	box-sizing: border-box;
	padding: 2px;
	height: 100vh;
	max-height: 100vh;
	width: 100vw;
	display: grid;
	grid-template:
		"main movable"
		"main header"
		"main footer"
		"main tauri";
	grid-template-columns: 1fr 4rem;
	grid-template-rows: 2.4rem 1fr 2.8rem 2.8rem;
	gap: 2px;
	&.nichts {
		grid-template-columns: 1fr 2.6rem;
	}
	> main {
		grid-area: main;
		overflow: auto;
	}
	> footer {
		grid-area: footer;
	}
	> header {
		grid-area: header;
		width: 100%;
		height: 100%;
	}
	> .movable {
		grid-area: movable;
	}
	> .tauri {
		grid-area: tauri;
	}
}
.haupt > .movable {
	display: flex;
	flex-direction: column;
	align-items: cernter;
	> div {
	}
	> .windowdragger {
		background-color: #eee;
		width: 100%;
		height: 100%;
		cursor: move;
		border: none;
		&:active {
			cursor: move;
			background-color: yellow;
		}
	}
}
.haupt {
	&.nichts {
		background-color: var(--hauptgegenhintergrundfarbe);
		color: var(--hauptgegenhintergrundfarbe);
		> main {
			background-color: var(--haupthintergrundfarbe);
		}
	}
	> .tauri {
		text-align: center;
		background-color: var(--haupthintergrundfarbe);
	}
}
.haupt {
	background-color: #002244;
	> main {
		background-color: white;
	}
	> footer {
		background-color: pink;
		> .aktionen {
			display: flex;
			padding: .4rem;
		}
	}
	> header {
		color: white;
		background-color: #034694;
		padding: .4rem;
		box-sizing: border-box;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		/* justify-content: center;  */
		/* Align horizontal */
		align-items: center;
		position: relative;
		> .app-name {
			font-size: 1.3rem;
			writing-mode: vertical-rl;
			flex-shrink: 1;
		}
		> .fahne {
			display: block;
			position: absolute;
			top: 0px;
			right: 0px;

			padding: .3rem;
			box-sizing: border-box;

			height: 1.8rem;
			width: 100%;
			text-align: center;
			font-size: .8rem;
			border: none;
			&.schreiben { background-color: orange; cursor: pointer; }
			&.default { background-color: #034694; }
			&.error { background-color: red; }
			&.pause { background-color: green; }
		}
		> .title {
 /* Align vertical */
			// background-color: yellow;
			position: absolute;
			top: 1.8rem;
			left: 3.3rem;
			width: calc(100vh - 10rem - 3px - 1.8rem);
			transform-origin: 0 0;
			transform: rotate(90deg);
			justify-content: center;
			text-align: center;
			font-size: 1.2rem;
			overflow: hidden;
			max-height: 6rem;
			display: gird;
			grid-template-rows: 3fr 1fr;
			> div {
				overflow: hidden;
    			white-space: nowrap;
    			text-overflow: ellipsis;
			}
			> .pfad {
				color: lightblue;
				font-size: .8rem;
				overflow: hidden;
				> span {
					padding: 0 .5rem;
				}
			}
		}
	}
	> .movable {
		background-color: #034694;
	}
}
.schreiber {
	margin: auto;
	max-width: 36rem;
	> .fake {
		display: grid;
		// background-color: #ccc;
		grid-template-columns: 34px 1fr;
		gap: 1px;
		> .line {
			// background-color: #f5f5f5;
		}
		> .editor {
			background-color: #fff;
		}
		> div {
			height: 50vh;
			width: 100%;
		}
	}
}

button.elemental {
	background-color: transparent;
	color: white;
	padding: .35rem;
	border: none;
	box-shadow: none;
	margin: .2rem;
	align-content: center;
	&:hover {
		box-shadow: none;
	}
}

footer {
	> .ueber {
		display: flex;
		flex-direction: column;
		> .sprache {
			padding: .2rem .5rem;
			text-align: center;

			color: darkred;
		}
	}
}

nav {
	margin: 0px;
	padding: 0px;
	ul {
		margin: 0px;
		padding: 0px;
		gap: 2px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		li {
			padding: 1rem .5rem;
			list-style: none;
			writing-mode: vertical-lr;
			color: #333;
			background-color: lightblue;
		}
	}
}
</style>
