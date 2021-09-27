<script>
	import { onMount } from "svelte"
	import { ui_theme, ui_custom_theme } from "./data"

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
		
		ui_theme.subscribe(() => { set_theme(); });
		ui_custom_theme.subscribe(() => { set_theme(); });
		
		// Get name of route from hash
		route = window.location.hash.substr(1)
		if (route == "") route = "launcher"
	});
	
	function set_theme() {
		// Set base theme
		document.documentElement.setAttribute('data-theme', $ui_theme ? '' : 'light');
		
		// Apply custom theme
		
		if ($ui_custom_theme = null) {
			console.log("Set call");
			// Loop through custom theme and set each root css property accordingly
			for (let [key, value] of Object.entries($ui_custom_theme)) {
				console.log(key, value);
				document.documentElement.style.setProperty(key, value);
			}
		}
	}

</script>

<main>
	<svelte:component this={routes[route]} />
</main>