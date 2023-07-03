<script lang="ts">
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import GridImage from "src/components/gridImage/gridImage.svelte";
  import VirtualGrid from "src/components/virtualGrid/virtualGrid.svelte";
  import Folder from "./folder.svelte";

  import { onDestroy } from "svelte";
  import { images_scroll_index } from "src/store/images";

  import type { ImageBucket, CheckedFile } from "src/types";
  export let bucket: ImageBucket;
  export let checkedFiles: CheckedFile[] = [];
  export let handleCheckbox;
  export let gridCol;

  let savedScroll: number;
  const _unsubscribeScroll = images_scroll_index.subscribe(
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
    images_scroll_index.update((store: { [key: string]: number }) => {
      let newStore = { ...store };
      newStore[bucket.name] = null;
      return newStore;
    });
  }

  onDestroy(() => {
    // Save scroll position
    images_scroll_index.update((store: { [key: string]: number }) => {
      let newStore = { ...store };
      newStore[bucket.name] = start;
      return newStore;
    });
  });

  let folderList: string[] = [];
  folderList = [
    ...new Set(
      bucket.files.map((file) => {
        return file.folder;
      })
    ),
  ];

  // Add folders list to checked folders list if not already there

  let checkedFolders: string[] = [];
  checkedFolders = [...folderList];

  let filteredFiles: ImageBucket = bucket;
  $: filteredFiles = {
    name: bucket.name,
    files: bucket.files.filter((file) => {
      return checkedFolders.includes(file.folder);
    }),
    total_files: bucket.total_files,
  };

  let handleCheckFolders: (folder: string) => void = (folder: string) => {
    if (checkedFolders.includes(folder)) {
      checkedFolders = checkedFolders.filter((f) => f !== folder);
    } else {
      checkedFolders = [...checkedFolders, folder];
    }
  };
</script>

<div class="mr-4">
  <NameDivider
    label={`bucket: ${bucket.name}
${bucket.files.length > 0 ? `(${bucket.files.length})` : ""}`}
  />
</div>
<div
  class="mr-4 mb-2 flex flex-auto gap-4 flex-wrap justify-center items-center border-2 border-orange-500"
>
  <div class="ml-2">folders:</div>
  <div class="flex flex-auto gap-4 flex-wrap">
    {#each folderList as folder (folder)}
      <Folder {checkedFolders} {handleCheckFolders} {folder} />
    {/each}
  </div>
</div>
<VirtualGrid
  items={filteredFiles.files}
  length={filteredFiles.files.length}
  bind:end
  bind:start
  bind:realMount
  {gridCol}
  bind:scrollToIndex
  let:gridCell
>
  {#each gridCell as i (i.key)}
    <GridImage
      {handleCheckbox}
      {checkedFiles}
      name={i.name}
      key={i.key}
      url={i.url}
      size={i.size}
      last_modified={i.last_modified}
      {bucket}
    />
  {/each}
</VirtualGrid>
