<script lang="ts">
  import { getTailwindClass, chunkify } from "src/lib/grid";
  import VirtualList from "src/components/virtualList/virtualList.svelte";
  import type { GridCol } from "src/types";
  const height = "calc(100vh - 160px)";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let items: any[];
  export let length: number;
  export let gridCol: GridCol;
  export let scrollToIndex = undefined;
  export let realMount = undefined;
  export let start = undefined;
  export let end = undefined;
</script>

<VirtualList
  items={chunkify(items, Number((length / gridCol).toFixed()), false)}
  {height}
  bind:scrollToIndex
  bind:realMount
  bind:end
  bind:start
  let:item
>
  <div class={`grid ${getTailwindClass(gridCol)} mr-4`}>
    <slot gridCell={item} />
  </div>
</VirtualList>
