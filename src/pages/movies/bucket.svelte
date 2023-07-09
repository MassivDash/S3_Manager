<script lang="ts">
  import NameDivider from "src/components/nameDivider/nameDivider.svelte";
  import GridVideo from "src/components/gridVideo/gridVideo.svelte";
  import VirtualGrid from "src/components/virtualGrid/virtualGrid.svelte";
  import { onDestroy } from "svelte";
  import { movies_scroll_index } from "src/store/movies";
  import { folder_movies } from "src/store/folder";
  import FolderPicker from "../../components/foldersPicker/foldersPicker.svelte";

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

  let folderList: string[] = [];
  folderList = [
    ...new Set(
      bucket.files.map((file) => {
        return file.folder;
      })
    ),
  ];

  let checkedFolders: string[] = [];

  const _unsubscribeFolder = folder_movies.subscribe(
    (value: { [key: string]: string[] }) => {
      if (value && value[bucket.name]) {
        checkedFolders = value[bucket.name];
      }
    }
  );

  $: if (realMount) {
    savedScroll && scrollToItem(savedScroll);
    movies_scroll_index.update((store: { [key: string]: number }) => {
      let newStore = { ...store };
      newStore[bucket.name] = null;
      return newStore;
    });
    if (checkedFolders.length === 0) {
      folder_movies.update((store: { [key: string]: string[] }) => {
        let newStore = { ...store };
        newStore[bucket.name] = [...folderList];
        return newStore;
      });
    }
  }

  onDestroy(() => {
    // Save scroll position
    movies_scroll_index.update((store: { [key: string]: number }) => {
      let newStore = { ...store };
      newStore[bucket.name] = start;
      return newStore;
    });
  });

  let filteredFiles: ImageBucket = bucket;
  $: filteredFiles = {
    name: bucket.name,
    files: bucket.files.filter((file) => {
      return checkedFolders.includes(file.folder);
    }),
    total_files: bucket.total_files,
  };

  let handleCheckFolders: (folder_name: string) => void = (
    folder_name: string
  ) => {
    if (checkedFolders.includes(folder_name)) {
      folder_movies.update((store: { [key: string]: string[] }) => {
        let newStore = { ...store };
        newStore[bucket.name] = checkedFolders.filter((f) => f !== folder_name);
        return newStore;
      });
    } else {
      folder_movies.update((store: { [key: string]: string[] }) => {
        let newStore = { ...store };
        newStore[bucket.name] = [...checkedFolders, folder_name];
        return newStore;
      });
    }
  };
</script>

<div class="mr-4 mt-2">
  <NameDivider
    label={`bucket: ${bucket.name}
  ${bucket.files.length > 0 ? `(${bucket.files.length})` : ""}`}
  />
</div>

<FolderPicker {checkedFolders} {handleCheckFolders} {folderList} />

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
