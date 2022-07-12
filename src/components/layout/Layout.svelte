<script lang="ts">
  import '../sideMenu/menu.svelte';
import SideMenu from '../sideMenu/menu.svelte';
</script>


<div class="flex w-full h-full">
  <div class="w-1/6">
  <SideMenu />
  </div>
  <main class="w-5/6 bg-slate-100">
    <slot />
  </main>       
</div>

