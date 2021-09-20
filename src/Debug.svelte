<script>
    import TitleBar from "./components/Titlebar.svelte";

    // UI kit debugging
    import DropDown from "./ui/Dropdown.svelte";
    import Drawer from "./ui/Drawer.svelte";

    let intefrace_options = [
        { name: `My example item`, calling: () => example_function() },
        { name: `Lorem Ipsum`, calling: () => example_function() },
        { name: `Last example item`, calling: () => example_function() }
    ];

    function example_function() {
        alert("Example function says hello!");
    }
</script>

<TitleBar window_name="UI kit debug">
</TitleBar>

<div style="padding:2em">
    <p>UI Buttons</p>
    <div class="flex row">
        <button on:click="{() => example_function()}" >Classic Button</button>
    </div>
    
    <div style="margin: 1em;"></div>
    <p>UI Dropdown</p>
    <div class="flex row">
        <DropDown options={intefrace_options}> <p>Example Dropdown</p> </DropDown>
        <div style="margin: 1em;"></div>
        <DropDown use_icon={false} options={intefrace_options}> <p>Dropdown With no Arrow</p> </DropDown>
    </div>

    <div style="margin: 1em;"></div>
    <p>UI Drawer</p>
    <div style="margin: 1em;"></div>
    <Drawer>
        <div style="margin: 2em;">
            <h1>Fancy Drawer stuff</h1>
        </div>
    </Drawer>
</div>