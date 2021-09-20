<script>
    import { fade, fly } from 'svelte/transition';

    export let lable = "Drawer";
    let triggerd = false;
</script>

<div class="drawer">
    <button class="flex row dbtn" on:click="{() => {triggerd = !triggerd}}">
        {lable}
        <div style="margin-left: auto;">
            <svg width="8" height="8" viewBox="0 0 8 8" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M0 3.6825H3.68248H7.36497L3.68248 7.36498L0 3.6825Z" fill="var(--ce-fg)"/>
            </svg>                
        </div>
    </button>
    {#if triggerd}
        <div in:fly="{{ y: -25, duration: 300, delay: 0 }}" class="content" >
            <slot></slot>
        </div>
    {/if}
</div>

<style>
    .drawer {
        border: 1px solid var(--ce-bg);
    }

    .content{
        padding: 0px 10px;
    }

    .dbtn {
        background-color: var(--ce-bg);
        width: 100% !important;
    }
    .dbtn:hover {
        background-color: var(--ce-bg-active);
    }
</style>