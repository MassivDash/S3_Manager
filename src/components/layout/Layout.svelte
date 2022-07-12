<script lang="ts">
  import '../sideMenu/sideMenu.svelte';
import SideMenu from '../sideMenu/sideMenu.svelte';
</script>


<div id="Layout">
  <SideMenu />
  <main>
    <slot />
  </main>       
</div>

<style>
  #Layout {
    color: red;
  }
</style>
