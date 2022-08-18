<script lang="ts">
  import IconButton from "src/components/iconButton/iconButton.svelte";
  import Search from "src/components/search/search.svelte";
  import Download from "src/components/icons/download.svelte";
  import Delete from "src/components/icons/delete.svelte";
  import Sync from "../icons/sync.svelte";
  import Grid from "../icons/grid.svelte";

  import { fade } from "svelte/transition";

  import type { CheckedFile } from "src/types";

  export let checkedFiles: CheckedFile[];
  export let handleDelete: (checkedFiles: CheckedFile[]) => void;
  export let handleDownload: (checkedFiles: CheckedFile[]) => void;

  export let handleSync: () => void | undefined = undefined;
  export let handleGrid: () => void | undefined = undefined;
  export let value: string;

  $: search = value;
  console.log(value);
</script>

<div
  class="fixed gap-4 flex w-[calc(100%-120px)] items-center h-20 top-0 right-4 bg-stone-100 text-gray-700 dark:text-white dark:bg-slate-900 z-30"
>
  {#if handleSync}
    <IconButton onClick={handleSync}>
      <Sync />
    </IconButton>
  {/if}
  {#if handleGrid}
    <IconButton onClick={handleGrid}>
      <Grid />
    </IconButton>
  {/if}
  <div class="justify-self-center flex-grow">
    <Search
      bind:value
      hidelabel="true"
      class="placeholder-gray-700 h-14 w-full border-orange-100  bg-orange-50 appearance-none outline-none border-2 border-transparent border-spacing-1  focus:border-orange-600 rounded text-gray-900 p-2"
    />
  </div>
  {#if checkedFiles && checkedFiles.length > 0}
    <div
      id="tool"
      class="transition-opacity gap-2 flex m-auto justify-self-end justify-end text-gray-600 ease-in duration-700"
      in:fade={{ duration: 700 }}
      out:fade={{ duration: 700 }}
    >
      <IconButton onClick={() => handleDownload(checkedFiles)}>
        Download
        <Download />
      </IconButton>
      <IconButton onClick={() => handleDelete(checkedFiles)}>
        Remove
        <Delete />
      </IconButton>
    </div>
  {/if}
</div>
