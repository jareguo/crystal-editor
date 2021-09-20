<script>
    import TitleBar from "./components/Titlebar.svelte";
    import DropDown from "./ui/Dropdown.svelte";
    import Drawer from "./ui/Drawer.svelte";

    let items = [
        {lable: "Darkmode", default: "", value: "darkmode"},
        {lable: "Darkmode", default: "", value: "darkmode"},
        {lable: "Darkmode", default: "", value: "darkmode"},
    ]

</script>

<TitleBar window_name="Editor Settings" />

<div style="padding:2em" class="flex col">
    <Drawer lable="Lorem Ipsum">
        {#each items as item}
            <div class="flex row i-center">
                <p>{item.lable}</p>
                <button>
                    <p>{item.lable}</p>
                </button>
            </div>
        {/each}
    </Drawer>
    <Drawer>
        {#each items as item}
            <div class="flex row i-center">
                <p>{item.lable}</p>
                <button>
                    <p>{item.lable}</p>
                </button>
            </div>
        {/each}
    </Drawer>
</div>