<script>
    // Example of per item, { name: `My example item`, calling: () => example_function() }
    export let options;
    export let use_icon = true;
    
</script>

{#if options}
    <div class="dropdown">
        <button class="flex row dropbtn j-center i-center">
            <slot />
            {#if use_icon}
                <div style="padding-left: 0.5em;">
                    <svg width="8" height="8" viewBox="0 0 8 8" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M0 3.6825H3.68248H7.36497L3.68248 7.36498L0 3.6825Z" fill="var(--ce-fg)"/>
                    </svg>                
                </div>
            {/if}
        </button>
        <div class="flex col dropdown-content">
            {#each options as item}
                <button on:click={() => item.calling()} class="index">{item.name}</button>
            {/each}
        </div>
    </div>
{/if}

<style>
    .index{
        text-align: left;
        padding: 0em 1em;
    }
    .dropdown {
        position: relative;
        display: inline-block;
    }

    .dropdown-content {
        display: none;
        position: absolute;
        background-color: var(--ce-bg);
        min-width: 160px;
        box-shadow: 0px 6px 8px 0px rgba(0, 0, 0, 0.2);
        z-index: 1;
    }
    .dropdown:hover .dropbtn{
        background-color: var(--ce-bg);
    }
    .dropdown:hover .dropdown-content {
        display: flex;
    }
</style>
