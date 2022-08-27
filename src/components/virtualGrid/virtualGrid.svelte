<script lang="ts">
  import { getTailwindClass, GridCol, chunkify } from "src/lib/grid";
  import VirtualList from "src/components/virtualList/virtualList.svelte";

  const height = "calc(100vh - 160px)";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let items: any[];
  export let length: number;
  export let gridCol: GridCol;
</script>

<VirtualList
  items={chunkify(items, Number((length / gridCol).toFixed()), false)}
  {height}
  let:item
>
  <div class={`grid ${getTailwindClass(gridCol)} mr-4`}>
    <slot gridCell={item} />
  </div>
</VirtualList>
