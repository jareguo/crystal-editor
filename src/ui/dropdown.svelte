<script>
    let items = [
		{ text: `Editor Settings`, calling: () => not_imprlemented() },
        { text: `Plugins` },
		{ text: `Check Update` }
	];

    function not_imprlemented() {
        alert("Not implemented");
    }
</script>

<div class="dropdown">
    <button class="dropbtn"><slot /></button>
    <div class="flex col dropdown-content">
        {#each items as item}
            <button class="index">{item.text}</button>
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

    .dropdown-content a {
        color: black;
        padding: 5px 5px;
        text-decoration: none;
        display: flex;
    }

    .dropdown-content a:hover {
        background-color: #f1f1f1;
    }

    .dropdown:hover .dropdown-content {
        display: flex;
    }
</style>
