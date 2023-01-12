<script lang="ts">
  import IconButton from "src/components/iconButton/iconButton.svelte";
  import Search from "src/components/search/search.svelte";
  import Download from "src/components/icons/download.svelte";
  import Delete from "src/components/icons/delete.svelte";
  import Sync from "../icons/sync.svelte";
  import Grid from "../icons/grid.svelte";

  import Loader from "../loader/loader.svelte";
  import IntermediateProgress from "../intermediateProgress/intermediateProgress.svelte";
  import { fade } from "svelte/transition";

  import type { CheckedFile } from "src/types";

  export let checkedFiles: CheckedFile[] | undefined = undefined;
  export let handleDelete: (checkedFiles: CheckedFile[]) => void | undefined =
    undefined;
  export let handleDownload: (checkedFiles: CheckedFile[]) => void | undefined =
    undefined;

  export let handleSync: (string: "load" | "sync") => void | undefined =
    undefined;
  export let handleGrid: () => void | undefined = undefined;
  export let value: string;
  export let resync: boolean;
</script>

<div
  class="gap-4 h-20 top-0 left-0 bg-stone-100 text-gray-700 dark:text-white dark:bg-slate-900 z-30"
>
  <div class="gap-4 flex items-center h-20">
    {#if handleSync}
      <IconButton onClick={() => handleSync("sync")}>
        {#if resync}
          <Loader width={24} height={24} />
        {:else}
          <Sync />
        {/if}
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
        class="placeholder-gray-500 h-14 w-full border-orange-100 text-gray-900 dark:text-white bg-orange-50 dark:bg-slate-800  dark:placeholder-white appearance-none outline-none border-2 border-transparent border-spacing-1  focus:border-orange-600 rounded  p-2"
      />
    </div>
    {#if checkedFiles && checkedFiles.length > 0}
      <div
        id="tool"
        class="transition-opacity mr-0 gap-4 flex text-gray-600 ease-in duration-700"
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
  <div class="-mt-4 r-2 flex justify-end">
    {#if resync}
      <IntermediateProgress />
    {/if}
  </div>
</div>
