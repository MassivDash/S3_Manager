<script lang="ts">
  import MenuItem from "./Item.svelte";
  const menuItems: {
    name: string;
    path: string;
    icon: string;
  }[] = [
    {
      name: "Home",
      path: "/",
      icon: "home",
    },
    {
      name: "All photos",
      path: "/photos",
      icon: "photo",
    },
    {
      name: "Buckets explorer",
      path: "/buckets",
      icon: "bucket",
    },
  ];
</script>

<header>
  <nav class="flex flex-col h-full">
    {#each menuItems as { name, path, icon }}
      <MenuItem {name} {path} {icon} />
    {/each}
  </nav>
</header>
