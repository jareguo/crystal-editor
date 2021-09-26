<script>
    import { fade, fly } from 'svelte/transition'

    export let padding = "10px";

    export let lable = "Drawer";
    let triggerd = false;
</script>

<div class="drawer">
    <button class="flex row dbtn" on:click="{() => {triggerd = !triggerd}}">
        {lable}
        <div style="margin-left: auto;">
            <svg width="8" height="8" viewBox="0 0 8 8" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M0 3.6825H3.68248H7.36497L3.68248 7.36498L0 3.6825Z" fill="var(--fg)"/>
            </svg>                
        </div>
    </button>
    {#if triggerd}
        <div in:fly="{{ y: -25, duration: 300, delay: 0 }}" style="padding:{padding};" class="drawer-content" >
            <slot></slot>
        </div>
    {/if}
</div>

<style>
    .drawer {
        border: 1px solid var(--bg);
    }

    .dbtn {
        background-color: var(--bg);
        width: 100% !important;
    }
    .dbtn:hover {
        background-color: var(--interactive-hover);
    }
</style>