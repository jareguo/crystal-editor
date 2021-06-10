<script>
    import TitleBar from "./components/Titlebar.svelte";
    import DropDown from "./ui/dropdown.svelte";

    let items = ["", ""];
</script>

<TitleBar window_name="Editor Settings">
    <DropDown {items}>Settings</DropDown>
</TitleBar>