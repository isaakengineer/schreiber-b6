<script>
	import { event } from "@tauri-apps/api";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";

	let file;
	let pfad = null;

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

	const allowDrop = (event) => { // nur zum Testen
		// console.log("something!")
	}
	function dragoverHandler(ev) {
	  ev.preventDefault();
	}
</script>

<header class="wilkomen-seite">
	<section>
		<p>Noch ist kein Pfad vorhanden.</p>
		<div style="display: flex; justify-content: space-between;">
			<button on:click={fileWaehlen}>Datei wählen</button>
       		<button on:click={fileErstellen}>Datei schaffen</button>
        </div>
	</section>
	<section>
	    <div class="box dropzone" on:drop={pfadLesen} on:dragover={allowDrop} on:dragover={dragoverHandler} >
	        <p>Bitte ziehen Sie eine Textdatei hier hinein.</p>
	    </div>
	</section>
    <section class="message">
        <header>Wilkommen auf Schreiber-B6</header>

        <nav>
        	<ul>
         		<li>Einstellung</li>
           		<li>Über</li>
             	<li>Dokumentation</li>
            </ul>
        </nav>
    </section>
</header>


<style lang="scss">
.wilkomen-seite {
    display: flex;
    flex-direction: column;
    // display: grid;
    // grid-template-rows: fit-content 1fr;
    height: 100%;
    .message {
    }
}
.box {
    margin: .2rem .4rem;
    // width: 100%;
    height: 50vh;
    // background-color: gray;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px dashed lightblue;
    color: var(hauptgegenhintergrundfarbe);
}
</style>
