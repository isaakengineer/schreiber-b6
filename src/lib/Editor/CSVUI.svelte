<script>
	import { csvDatei } from '$lib/Editor/store.js';
	import { Ausstattung } from '../../routes/store';
</script>

<table class="datei">
	<thead class="kopfreihe">
		{#each $csvDatei.kopfreihe as k}
			<th class="">{k}</th>
		{/each}
	</thead>
	<tbody class="reihen">
		{#each $csvDatei.reihen as r}
			<tr class="reihe">
				{#each $csvDatei.kopfreihe as k}
					<td class="" >{r[k]}</td>
				{/each}
			</tr>
		{/each}
		<tr class="reihe" style="grid-template-columns: repeat(auto-fit, minmax(3rem, 1fr))">
			{#each $csvDatei.kopfreihe as k, index}
				<td class="" >
					<input bind:value={$csvDatei.neueReihe[index]} on:input={() => {
						$csvDatei.fahneGeaendert = true;
						$Ausstattung.editorStatus = 'schreiben';
					}} />
				</td>
			{/each}
		</tr>
	</tbody>
</table>

<style lang="scss">
table {
	border-collapse: collapse;
	border: 1px solid #ccc;
	input {
		padding: .5rem;
		width: fit-content;
		width: 100%;

		box-sizing: border-box;
		border-radius: 0 !important;
	}
	> thead {
		top: 0px;
		position: sticky;
		font-size: .92rem;

	}
	th, td {
		border: 1px solid #ddd;
		padding: 8px;
		text-align: left;
		width: fit-content;
	}

	th {
		background-color: #f2f2f2;
		text-align: center;
	}
	tbody {
		font-size: .9rem;
	}
	tr:last-child {
		> td {
	 		padding: 0px;
			box-sizing: border-box;
		}
	}
}
</style>
