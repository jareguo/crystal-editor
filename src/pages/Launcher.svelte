<script>
    import TitleBar from '../components/Titlebar.svelte'
    import DropDown from '../ui/Dropdown.svelte'
    import Settings from './Settings.svelte'

    import { WebviewWindow } from "@tauri-apps/api/window";

    function createWindow(window_title, page_name) {
        const webview = new WebviewWindow(window_title, {url: "#" + page_name, "decorations": false});
        //windowMap[window_title] = webview;
        webview.once('tauri://error', function () {
            alert('Error loading page');
        })
    }

    // Trigger opening project
    function openProject(project) {
        console.log('Opening project', project);
        createWindow(project.name, 'editor');
    }
    
    let settings_dropdown = [{name: "Editor Settings", calling: () => { createWindow("Settings", "settings") }}]
    let debug_dropdown = [{name: "UI Debug", calling: () => { createWindow("Debug", "debug") }}]

    let projects = [{name: "Debug Inteface Dev", engine: "Engine Name"}]
</script>

<TitleBar window_name="Project Launcher" can_maximize="{false}">
    <DropDown use_icon={false} options={settings_dropdown}>Settings</DropDown>
    <DropDown use_icon={false} options={debug_dropdown}>Debug</DropDown>
</TitleBar>

<div class="frame flex col">
<h1>Projects</h1>
    <div class="projects row">
        {#each projects as item}
        <button class="project flex col j-center i-center" on:click="{() => openProject(item)}">
            <img
            src="/icons/editor.svg"
            height="80"
            alt="engine"
            />
            <div class="no-clicks" style="padding-top:30px">
                <h3><strong>{item.name}</strong></h3>
                <div style="margin:0.4em"></div>
                <p class="small">{item.engine}</p>
            </div>
        </button>
        {/each}
    </div>
</div>

<div class="bar flex row space i-center">
    <p class="v-center small">V. 0.10</p>
    <div>
        <button>Open External</button>
        <button>New Project</button>
    </div>
</div>

<style>
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
