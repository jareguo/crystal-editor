<script>
	import { onMount } from "svelte"

	// Route imports
	import Editor from './Editor.svelte'
	import Launcher from './Launcher.svelte'
	import Settings from './Settings.svelte'

	// Hash routes
	let routes = { "settings" : Settings, "editor" : Editor, "launcher" : Launcher }

	let route;
	onMount(() => {
		// Get name of route from hash
		route = window.location.hash.substr(1)
		if (route == "") route = "launcher"
	});
	
</script>

<main>
	<svelte:component this={routes[route]} />
</main>