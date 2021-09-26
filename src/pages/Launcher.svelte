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
            <svg width="80" height="80" viewBox="0 0 176 176" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M25.6941 26.1745C42.1457 9.7229 64.4589 0.480469 87.725 0.480469C110.991 0.480466 133.304 9.72289 149.756 26.1745C166.208 42.6262 175.45 64.9394 175.45 88.2055C175.45 111.472 166.208 133.785 149.756 150.236C144.368 155.625 138.351 160.239 131.878 164.009L126.552 136.309L146.53 97.6187L145.139 57.0297L144.636 57.3811L145.094 56.9894L121.496 24.8148L106.755 17.0962L91.1992 20.2033L103.095 64.8672L83.6268 76.8693L85.5918 79.4048L106.387 67.4317L118.541 65.0777L108.165 63.4942L98.6853 26.8129L108.09 25.9503L120.687 64.4835L48.4749 111.494L29.4912 112.056L31.341 119.117L69.2336 140.738L76.2768 175.181C57.2356 172.675 39.4237 163.966 25.6941 150.236C9.24244 133.785 1.01789e-05 111.472 0 88.2055C9.79358e-06 64.9394 9.24244 42.6262 25.6941 26.1745ZM82.0787 175.749C83.9528 175.87 85.8357 175.93 87.725 175.93C88.997 175.93 90.2661 175.903 91.5316 175.848L82.9317 137.053L73.1554 140.415L82.0787 175.749ZM82.9262 137.029L82.9256 137.026L50.4049 118.989L35.2858 117.554L70.4146 136.03L82.9262 137.029ZM105.866 174.035C109.921 173.177 113.902 172.036 117.781 170.621L115.725 157.622L108.282 152.253L103.69 160.861L105.866 174.035ZM108.205 86.6555L109.272 85.908L117.801 81.465L122.721 101.298L123.021 103.254L122.822 103.387L121.012 103.051L111.954 102.685L108.205 86.6555ZM104.728 89.0916L89.7011 99.6197L93.3897 113.454L94.3971 112.876L108.459 103.019L104.728 89.0916ZM94.4213 117.323L95.5753 121.651L118.761 106.109L110.666 105.835L94.4213 117.323ZM37.004 100.614L55.994 100.063L79.3485 83.9874L65.5916 32.4093L49.3629 36.7792L37.004 100.614Z" fill="var(--fg)"/>
                </svg>                
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
        color: var(--fg1);
    }
    .bar{
        padding: 1em;
        background-color: var(--bg);
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
        background-color: var(--bg);
        border-radius: 0.6em;
    }
    .project:hover{
        background-color: var(--interactive-hover);
    }

    hr {
        border: solid 1px #323232;
        width: 100%;
    }

    /* Force a invert of the background in the launcher */
    :global(html) {
        background-color: var(--bg1) !important;
    }
</style>
