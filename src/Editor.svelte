<script>
    import TitleBar from "./components/Titlebar.svelte";
    import DropDown from "./ui/dropdown.svelte";

    let items = ["", ""];
</script>

<TitleBar>
    <DropDown {items}>Settings</DropDown>
</TitleBar>

<h1>Editor intefrace</h1>