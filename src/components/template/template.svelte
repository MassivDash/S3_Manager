<script lang="ts">
  import "../sideMenu/menu.svelte";
  import SideMenu from "../sideMenu/menu.svelte";
</script>

<div class="flex w-full flex-row h-screen overflow-hidden">
  <div class="relative w-24">
    <SideMenu />
  </div>
  <main class="w-full bg-stone-100 overflow-y-auto p-4"> 
    <slot />
  </main>
</div>
