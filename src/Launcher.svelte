<script>
    import TitleBar from './components/Titlebar.svelte'
    import DropDown from './ui/dropdown.svelte'
    
    let items = [{name: "Game Project Name", engine: "Amethyst 0.16"}, {name: "Lorem Ipsum", engine: "Minigene 0.2"}]
</script>

<TitleBar window_name="Project Launcher">
    <DropDown items={items}>Settings</DropDown>
</TitleBar>

<div class="frame flex col">
<h1>Projects</h1>
    <div class="projects row">
        {#each items as item}
        <button class="project">
            <img
            class="icon"
            style="padding:10px 50px"
            src="/icons/editor.svg"
            height="80"
            alt="engine"
            />
            <div class="no-clicks" style="padding:10px">
                <hr />
                <h3><strong>{item.name}</strong></h3>
                <p class="small">{item.engine}</p>
            </div>
        </button>
        {/each}
    </div>
</div>

<div class="bar flex row space">
    <p class="v-center small">V. 0.10</p>
    <div>
        <button>Open External</button>
        <button>New Project</button>
    </div>
</div>

<style>
    .v-center{
        margin: auto 0 !important;
    }
    .small{
        color: var(--ce-fg1);
    }
    .bar{
        padding: 1em;
        background-color: var(--ce-bg);
        position:fixed;
        bottom: 0px;
        left: 0;
        right: 0;
        font-size: 13px;
    }

    .frame {
        padding: 2em;
    }

    .projects{
        margin-top: 1em;
    }

    .project {
        margin-right: 2em;
        margin-bottom: 2em;
        width: 14em;
        height: 15em;
        padding: 0em;
        box-shadow: 0px 1px 6px rgba(0, 0, 0, 0.25);
        text-align: left;
        background-color: var(--ce-bg);
        border-radius: 0.6em;
    }
    .project:hover{
        background-color: var(--ce-bg1);
    }

    hr {
        border: solid 1px #323232;
        width: 100%;
    }

    /* Force a invert of the background in the launcher */
    :global(html) {
        background-color: var(--ce-bg1) !important;
    }
</style>
