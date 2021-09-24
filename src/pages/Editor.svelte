<script>
	import { onMount } from 'svelte';
    import TitleBar from "../components/Titlebar.svelte";
    import DropDown from "../ui/Dropdown.svelte";

    const invoke = window.__TAURI__.invoke;

    // Editor instance init
    invoke('editor_instance_init');

</script>

<TitleBar>
</TitleBar>
