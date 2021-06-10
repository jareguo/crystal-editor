<script>
	import { WebviewWindow } from "@tauri-apps/api/window";

	import Editor from './Editor.svelte'
	import Launcher from './Launcher.svelte'

	let is_Launcher = true;

	function create_window(){
		const webview = new WebviewWindow('theUniqueLabel', {
			url: 'path/to/page.html'
		})
	}

</script>

<main>
	{#if is_Launcher}
		<Launcher />
	{:else}
		<Editor />
	{/if}
</main>