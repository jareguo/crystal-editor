<script>
	//import { appWindow } from "@tauri-apps/api/window";

	import Editor from './Editor.svelte'
	import Launcher from './Launcher.svelte'

	let is_Launcher = true;

</script>

<main>
	{#if is_Launcher}
		<Launcher />
	{:else}
		<Editor />
	{/if}
</main>