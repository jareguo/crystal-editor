<script>
    import { exit } from "@tauri-apps/api/process";
    import { appWindow } from "@tauri-apps/api/window";
    const {
        setResizable,
        setTitle,
        maximize,
        unmaximize,
        minimize,
        unminimize,
        show,
        hide,
        setDecorations,
        setAlwaysOnTop,
        setSize,
        setMinSize,
        setMaxSize,
        setPosition,
        setFullscreen,
        setIcon,
    } = appWindow;

    let maximized = true;

    function request_minimize(){
        minimize();
    }
    function request_maximize(){
        maximized ? maximize() : unmaximize();
        maximized = !maximized;
    }
    function request_exit(){
        // We are going to call rust and tell the Crystal-Hook we are exiting.
        
        exit(0);
    }

</script>

<div data-tauri-drag-region class="titlebar flex f-row f-space noselect">
    <div class="flex f-row">
        <img style="margin: auto 10px;" src="icons/editor.svg" height="20" alt="">
        <p class="v-center">Crystal Editor</p>
    </div>
    <div class="flex f-row">
        <button on:click={() => request_minimize()}>
            <img style="margin: auto 10px;" src="icons/tb_min.svg" height="11" width="11" alt="">
        </button>
        <button on:click={() => request_maximize()}>
            <img style="margin: auto 10px;" src="icons/tb_max.svg" height="11" width="11" alt="">
        </button>
        <button on:click={() => request_exit()} class="exit">
            <img style="margin: auto 10px;" src="icons/tb_exit.svg" height="11" width="11" alt="">
        </button>
    </div>
</div>

<style>
    button{
        min-width: 46px;
        border: 3px;
    }
    .exit:hover{
        background-color: var(--ce-error);
    }
</style>