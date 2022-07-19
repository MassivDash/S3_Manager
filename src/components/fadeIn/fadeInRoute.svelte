<script lang="ts">
  import { fade } from "svelte/transition";
  import { Route } from "svelte-navigator";
  export let path;
</script>

<Route {path}>
  <div in:fade>
    <slot />
  </div>
</Route>
