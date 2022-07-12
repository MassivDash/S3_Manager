<script lang="ts">
  import '../sideMenu/SideMenu.svelte';
import SideMenu from '../sideMenu/SideMenu.svelte';
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
