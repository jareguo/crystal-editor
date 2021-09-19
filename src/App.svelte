<script>
	import { WebviewWindow } from "@tauri-apps/api/window";

	import Editor from './Editor.svelte'
	import Launcher from './Launcher.svelte'
	import { onMount } from "svelte"

	onMount(() => {
		path = window.location.href;
	});

	let is_Launcher = true;
	let path = "";

	function create_window(){
		const webview = new WebviewWindow('theUniqueLabel', {
			url: 'path/to/page.html'
		})
	}

</script>

<main>
	<h1>{path}</h1>
	{#if is_Launcher}
		<Launcher />
	{:else}
		<Editor />
	{/if}
</main>