<script>
	import { invoke } from "@tauri-apps/api/core";
	import { emit, once, listen } from "@tauri-apps/api/event";
	import { getCurrentWindow } from '@tauri-apps/api/window';

				
	import { onMount } from 'svelte'
	import CodeMirror, { basicSetup } from '../Editor/CodeMirror.svelte'
	
	import Datei from '../datei/Datei.svelte';
	
	import XCircle from "phosphor-svelte/lib/XCircle";
	
	let store;
	let initialStore;
	let Ausstattung = {
		haupt: "nichts",
		identitaet: {
			
		}
	};
	let elementalCloseButtonWeight = "duotone";
	const appWindow = getCurrentWindow();
	
	onMount(async () => {
  		
		const cl = document.body.classList
		if (cl.contains('dark')) {
			cl.remove('dark')
		  return () => void cl.add('dark')
		}
	})
	
	listen("datei-gewaehlt", async (event) => {
		console.log("file gewählt")
		console.log(event.payload)
		Ausstattung.identitaet = event.payload;
		Ausstattung.haupt = "datei";
		console.log(Ausstattung.identitaet);
		initialStore = await invoke("lesen");
	});
	
	function changeHandler({ detail: {trs} }) {
		const changes = trs.map(tr => tr.changes)
		console.log('change', changes.map(ch => ch.toJSON()))
		if (changes.length > 1) {
			const totalChange = changes.reduce((c1, c2) => c1.compose(c2))
			console.log('totalChange', totalChange.toJSON())
		}
	}
	
	const speichern = async () => {
		await invoke("schreiben", { text: $store });
	}
</script>

<div class="haupt">
	<header>
		{#if Ausstattung.identitaet && Ausstattung.identitaet.name}
			<div class="title">
				<div class="name">{ Ausstattung.identitaet.name }</div>
				<div class="pfad">{ Ausstattung.identitaet.dateipfad }</div>
			</div>
		{:else}
			<div class="app-name">(Schreiber)</div>
		{/if}
	</header>
	<div class="movable">
		<button class="windowdragger" data-tauri-drag-region></button>
		<button class="elemental titlebar-button" id="titlebar-close"
				on:mouseover={() => elementalCloseButtonWeight = "fill"}
				on:mouseleave={() => elementalCloseButtonWeight = "duotone"}
				on:click={() => appWindow.close()}>
				<XCircle size="1.8em" bind:weight={elementalCloseButtonWeight} />
			</button>
	</div>
	<main>
		{#if Ausstattung.haupt === "nichts"}
			<Datei />
		{:else}
		<div class="schreiber">
			<CodeMirror 
				doc={initialStore}
				bind:docStore={store}
				extensions={basicSetup}
				on:change={changeHandler}></CodeMirror>
			<div class="fake">
				<div class="line"></div>
				<div class="editor"></div>
			</div>
		</div>
		{/if}
	</main>
	<footer>
		<div class="ueber"></div>
		<div class="aktionen">
			<button on:click={speichern}>speichere</button>
		</div>
	</footer>
</div>

<style lang="scss">
:global(.codemirror) {
	// border: 1px solid #ddd;
}
.haupt {
	box-sizing: border-box;
	padding: 1px;
	height: 100vh;
	max-height: 100vh;
	width: 100vw;
	display: grid;
	grid-template: "main header"
		"footer movable";
	grid-template-columns: 1fr 4rem;
	grid-template-rows: 1fr 6rem;
	gap: 1px;
	> main {
		grid-area: main;
		overflow: scroll;
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
	background-color: #002244;
	> main {
		background-color: white;
	}
	> footer {
		background-color: #eee;
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
		display: flex;
		justify-content: center; /* Align horizontal */
		align-items: center;
		font-size: 1.3rem;
		position: relative;
		> .app-name {
			writing-mode: vertical-rl;
		}
		> .title {
 /* Align vertical */
			// background-color: yellow;
			position: absolute;
			top: 0px;
			left: 3.3rem;
			transform-origin: 0 0;
			transform: rotate(90deg);
			justify-content: center;
			width: calc(100vh - 6rem - 3px);
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
	padding: .2rem;
	border: none;
				box-shadow: none;
				margin: .2rem;
				align-content: center;
				&:hover {
					box-shadow: none;
				}
			}
</style>