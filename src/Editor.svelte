<script>
    import TitleBar from "./components/Titlebar.svelte";
    import DropDown from "./ui/dropdown.svelte";

    let items = ["", ""];
</script>

<TitleBar>
    <DropDown {items}>Settings</DropDown>
</TitleBar>

<h1>Crystal Editor</h1>
<img src="icons/crystalhook.svg" class="box" height="32" alt="" />
<p>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Corporis, ab vitae.
    Expedita repudiandae cumque atque aliquam exercitationem harum fugit unde
    eligendi amet rem praesentium qui, cupiditate, ullam quisquam earum
    distinctio.
</p>
<hr />
<div class="code">
    <p>
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Corporis, ab
        vitae. Expedita repudiandae cumque atque aliquam exercitationem harum
        fugit unde eligendi amet rem praesentium qui, cupiditate, ullam quisquam
        earum distinctio.
    </p>
</div>
