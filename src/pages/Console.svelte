<script>
    import TitleBar from "../components/Titlebar.svelte";

    let history_max = 1000;
    let history = [];

</script>

<TitleBar window_name="Console" />

<div>
    {#each history as index}
        {index}
    {/each}
</div>