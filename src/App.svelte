<script>
	import { onMount } from "svelte"

	// Route imports
	import Editor from './Editor.svelte'
	import Launcher from './Launcher.svelte'
	import Settings from './Settings.svelte'
	import Debug from './Debug.svelte';

	// Hash routes
	let routes = { "settings" : Settings, "editor" : Editor, "launcher" : Launcher, "debug" : Debug }

	let route;
	onMount(() => {
		// TODO: Track what routes / windows are open and focus if duplicate.

		// Get name of route from hash
		route = window.location.hash.substr(1)
		if (route == "") route = "launcher"
	});

</script>

<main>
	<svelte:component this={routes[route]} />
</main>