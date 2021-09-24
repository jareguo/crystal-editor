<script>
	import { onMount } from "svelte"

	// Route imports
	import Editor from './pages/Editor.svelte'
	import Launcher from './pages/Launcher.svelte'
	import Settings from './pages/Settings.svelte'
	import Debug from './pages/Debug.svelte'
	import Console from './pages/Console.svelte'

	// Hash routes
	let routes = { "settings" : Settings, "editor" : Editor, "launcher" : Launcher, "debug" : Debug, "console": Console }

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