<script>
    import TitleBar from "../components/Titlebar.svelte"
    import DropDown from "../ui/Dropdown.svelte"
    import Drawer from "../ui/Drawer.svelte"
    import { ui_theme } from "../data"

    let custom_theme = [ 
        { key: "--bg", value: "#232323" }, 
        { key: "--fg", value: "rgb(223, 223, 223)" },
        { key: "--interactive-hover", value: "rgba(255, 255, 255, 0.13)" },
        { key: "--bg1", value: "#323232" },
        { key: "--fg1", value: "rgba(223, 223, 223, 0.418)" },
    ];

    // If the custom_theme is updated, update the theme
    function update_param(key, new_value) {
        custom_theme[key] = new_value;
        $ui_theme == custom_theme;
    }

</script>

<TitleBar window_name="Editor Settings" />

<div style="padding:2em" class="flex col">
    <Drawer lable="User Interface">
        <button class="index flex row space w-max" on:click="{() => $ui_theme = !$ui_theme}">
                <p>Dark Mode</p>
                <svg width="15" height="15" viewBox="0 0 124 124" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M62 124C96.2417 124 124 96.2417 124 62C124 27.7584 96.2417 0 62 0C27.7583 0 0 27.7584 0 62C0 96.2417 27.7583 124 62 124ZM62 105.4C85.9691 105.4 105.4 85.9691 105.4 62C105.4 38.0309 85.9691 18.6 62 18.6V105.4Z" fill="var(--fg)"/>
                </svg>
        </button>
        <Drawer lable="Custom Theme">
            {#each custom_theme as item}
                <div class="flex row space w-max">
                    <p class="w-max">{item.key}</p>
                    <div class="flex row w-max">
                        <input type="text" class="w-max" value="{item.value}" on:change="{(e) => update_param(item.key, e.target.value)}"/>
                        <input type="color" value="{item.value}" on:change="{(e) => update_param(item.key, e.target.value)}"/>
                    </div>
                </div>
            {/each}
        </Drawer>
    </Drawer>
</div>