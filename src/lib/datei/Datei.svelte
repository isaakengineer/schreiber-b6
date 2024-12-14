<script>
	import { event } from "@tauri-apps/api";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";

	let file;
	let pfad = null;
	let neueDatei = {
		name: '',
		inhalt: '',
	}
	let meldung = '';

	// Ein Funktionen-Paar, weil ansonsten Tauri kann nicht durch Browser die Pfad von "drag-drop event" lesen.
	listen('tauri://drag-enter', async (event) => {
		console.log("drag enter event", event);
		pfad = event.payload.paths[0];
	});
	listen('tauri://drag-leave', event => {
		console.log("dragged file left!")
		pfad = null;
	});
	listen('tauri://drag-drop', (event) => {
	console.log("tauri drop event!")
		console.log(event);
	})

 	window.addEventListener("drop",function(e){
		e = e || event;
		e.preventDefault();
	},false); //preventing drag and drop nonesense!

	async function fileWaehlen() {
			file = await invoke("datei_waehlen");
	}
	const fileErstellen = async () => {
			file = await invoke("datei_erstellen");
			console.log(file);
	}

	const pfadLesen = async (event) => {
		if (pfad) {
			file = await invoke("dateipfad_eingegeben", {
			pfad: pfad // hier die Pfad wird durch von Tauri festgelegten Data erfüllt
		});
	 	} else {
			console.log("something went wrong during drop!", event);
		}
	}

	const neueDateiErstellen = async () => {
		let status = await invoke('neue_datei_erstellen', { name: neueDatei.name, inhalt: neueDatei.inhalt }).catch((e) => {
			console.warn("neue Datei geschafft.", e);
			meldung = e;
		})
	}
	const allowDrop = (event) => { // nur zum Testen
		// console.log("something!")
	}
	function dragoverHandler(ev) {
		ev.preventDefault();
	}
</script>

<div class="wilkomen-seite">
	<header>
		<!-- Wilkommen auf -->
		<em>Schreiber-B6</em>
	</header>
	{#if meldung}
		<section class="meldung"><p>{meldung}</p></section>
	{/if}
	<section>
		<div>
			<p class="hinweis">Eine neue Datei in Ihre <strong>Dokumente</strong>fach</p>
		</div>
		<div class="input">
			<label for="inhalt">Inhalt</label>
			<textarea id="inhalt"
				size="2"
				placeholder="" bind:value={neueDatei.inhalt} />
		</div>
		<div class="input">
			<label for="name">Name</label>
			<input id="name" type="text"
				style="width: 100%;"
				placeholder="<Dateiname>.Endung"
				bind:value={neueDatei.name} />
			<button on:click={neueDateiErstellen}>erstellen</button>
		</div>
	</section>
	<section>
		<!-- <p class="hinweis">Noch ist kein Pfad vorhanden.</p> -->
		<div style="display: flex; justify-content: space-between;">
			<button on:click={fileWaehlen}>Datei wählen</button>
			 		<!-- <button on:click={fileErstellen}>Datei schaffen</button> -->
				</div>
	</section>
	<section class="box-container">
			<div class="box dropzone" on:drop={pfadLesen} on:dragover={allowDrop} on:dragover={dragoverHandler} >
					<p>Bitte ziehen Sie eine Textdatei hier hinein.</p>
			</div>
	</section>
</div>


<style lang="scss">
section {
	padding: 0px 1rem;
	&.meldung {
		margin: 1rem 1rem 0px 1rem;
		padding: 0px 1rem;
		background-color: pink;
		color: darkred;
		box-shadow: 0px 0px .2rem black;
		border-radius: .5rem;
	}
}
p.hinweis {
	color: lightblue;
}
section > .input {
	padding: .5rem;
	display: flex;
	label {
		margin-right: 1rem;
		padding: .5rem 0px;
	}
	input, textarea {
		padding: .5rem;
		font-size: 1rem;
	}
	textarea {
		flex-grow: 1;
		font-weight: 300;
		resize: none;
		/* font-family: 'Lato'; */
	}
}
.wilkomen-seite {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		// display: grid;
		// grid-template-rows: fit-content 1fr;
		height: 100%;
		> header {
			padding: 2rem 2rem 0px 2rem;
		font-size: 2em;
	}
		.message {
		}
	> .box-container { flex-grow: 1; padding: 1rem; padding-top: 0px; }
}
.box {
		// width: 100%;box-container
		// background-color: gray;
	display: flex;
	height: 100%;
	align-items: center;
		justify-content: center;
		border: 2px dashed lightblue;
		color: var(hauptgegenhintergrundfarbe);
}
</style>
