<script lang="ts">
  import MenuItem from "./sideItem.svelte";

  const menuItems: {
    name: string;
    path: string;
  }[] = [
    {
      name: "Home",
      path: "/",
    },
    {
      name: "All photos",
      path: "/photos"
    },
    {
      name: "Buckets explorer",
      path: "/buckets"
    },
  ];
</script>

<header>
  <nav>
    {#each menuItems as item}
      <MenuItem name={item.name} path={item.path} />
    {/each}
  </nav>
</header>

