<script>
    import { exit } from "@tauri-apps/api/process";
    import { appWindow } from "@tauri-apps/api/window";
    const {
        maximize,
        unmaximize,
        minimize,
    } = appWindow;

    let maximized = true;
    export let window_name = "Crystal Editor";

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

<div data-tauri-drag-region class="titlebar flex row space noselect">
    <div class="flex row">
        <img style="margin: auto 5px;" src="icons/editor.svg" height="23" alt="">
        <slot></slot>
    </div>
    <p class="v-center tb_title">{window_name}</p>
    <div class="flex row">
        <button on:click={() => request_minimize()}>
            <img class="v-center" src="icons/tb_min.svg" height="11" width="11" alt="">
        </button>
        <button on:click={() => request_maximize()}>
            {#if maximized}
                <img class="v-center" src="icons/tb_max.svg" height="11" width="11" alt="">
            {:else}
                <img class="v-center" src="icons/tb_max_min.svg" height="11" width="11" alt="">
            {/if}
        </button>
        <button on:click={() => request_exit()} class="exit">
            <img class="v-center" src="icons/tb_exit.svg" height="11" width="11" alt="">
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
    .tb_title{
        margin-left: calc(135px / 2);
    }
</style>