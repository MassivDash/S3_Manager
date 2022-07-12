<script lang="ts">
  import { Link } from "svelte-navigator";
  export let path;
  export let name;
</script>

<Link to={path}>
    <div class="sideItem">{name}</div>
</Link>

<style>
    .sideItem {
        color: red;
    }
</style>
