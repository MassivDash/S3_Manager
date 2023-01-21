<script lang="ts">
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import GridVideo from "src/components/gridVideo/gridVideo.svelte";
  import VirtualGrid from "src/components/virtualGrid/virtualGrid.svelte";
  import { onDestroy } from "svelte";
  import { movies_scroll_index } from "src/store/movies";

  import type { ImageBucket, CheckedFile } from "src/types";
  export let bucket: ImageBucket;
  export let checkedFiles: CheckedFile[] = [];
  export let handleCheckbox;
  export let gridCol;

  let savedScroll: number;
  const _unsubscribeScroll = movies_scroll_index.subscribe(
    (value: { [key: string]: number }) => {
      if (value && value[bucket.name]) {
        savedScroll = value[bucket.name];
      }
    }
  );

  // End item index needed for scroll restore
  let start: number | undefined; // first in view
  let end: number | undefined; // last in view

  // Save scroll position

  let scrollToIndex: (number: number) => void;
  function scrollToItem(number: number): void {
    scrollToIndex(number);
  }

  // virtual list mounted
  let realMount;

  $: if (realMount) {
    savedScroll && scrollToItem(savedScroll);
    movies_scroll_index.update((store: { [key: string]: number }) => {
      let newStore = { ...store };
      newStore[bucket.name] = null;
      return newStore;
    });
  }

  onDestroy(() => {
    // Save scroll position
    movies_scroll_index.update((store: { [key: string]: number }) => {
      let newStore = { ...store };
      newStore[bucket.name] = start;
      return newStore;
    });
  });
</script>

<div class="mr-6">
  <NameDivider
    label={`bucket: ${bucket.name}
  ${bucket.files.length > 0 ? `(${bucket.files.length})` : ""}`}
  />
</div>
<VirtualGrid
  items={bucket.files}
  length={bucket.files.length}
  bind:end
  bind:start
  bind:realMount
  {gridCol}
  bind:scrollToIndex
  let:gridCell
>
  {#each gridCell as i (i.key)}
    <GridVideo
      {handleCheckbox}
      {checkedFiles}
      name={i.name}
      key={i.key}
      url={i.url}
      size={i.size}
      {bucket}
    />
  {/each}
</VirtualGrid>