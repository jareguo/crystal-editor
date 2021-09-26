<script>
    import TitleBar from "../components/Titlebar.svelte";
    import { ui_theme } from "../data";

    // UI kit debugging
    import DropDown from "../ui/Dropdown.svelte";
    import Drawer from "../ui/Drawer.svelte";

    let intefrace_options = [
        { name: `My example item`, calling: () => example_function() },
        { name: `Lorem Ipsum`, calling: () => example_function() },
        { name: `Last example item`, calling: () => example_function() },
    ];

    function example_function() {
        alert("Example function says hello!");
    }
</script>

<TitleBar window_name="UI kit debug" />

<div style="padding:2em">
    <button
        class="index flex row space w-max"
        on:click={() => ($ui_theme = !$ui_theme)}
    >
        <p>Dark Mode</p>
        <svg
            width="15"
            height="15"
            viewBox="0 0 124 124"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M62 124C96.2417 124 124 96.2417 124 62C124 27.7584 96.2417 0 62 0C27.7583 0 0 27.7584 0 62C0 96.2417 27.7583 124 62 124ZM62 105.4C85.9691 105.4 105.4 85.9691 105.4 62C105.4 38.0309 85.9691 18.6 62 18.6V105.4Z"
                fill="var(--fg)"
            />
        </svg>
    </button>

    <p>UI Buttons</p>
    <div class="flex row">
        <button on:click={() => example_function()}>Classic Button</button>
    </div>

    <p>UI Dropdown</p>
    <div class="flex row">
        <DropDown options={intefrace_options}><p>Example Dropdown</p></DropDown>
        <div style="margin: 1em;" />
        <DropDown use_icon={false} options={intefrace_options}>
            <p>Dropdown With no Arrow</p>
        </DropDown>
    </div>

    <p>UI Drawer</p>
    <Drawer>
        <div style="margin: 2em;">
            <h1>Fancy Drawer stuff</h1>
        </div>
    </Drawer>
</div>
