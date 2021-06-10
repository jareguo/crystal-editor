<script>

    // Example of per item, { name: `My example item`, calling: () => example_function() }
    export let items = [];

    function example_function(){
        alert("example function");
    }
</script>

<div class="dropdown">
    <button class="dropbtn"><slot /></button>
    <div class="flex col dropdown-content">
        {#each items as item}
            <button on:click={() => item.calling()} class="index">{item.name}</button>
        {/each}
    </div>
</div>

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
