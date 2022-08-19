<script lang="ts">
  import Folder from "../icons/folder.svelte";
  import IconButton from "../iconButton/iconButton.svelte";
  import Fold from "../icons/fold.svelte";
  import Unfold from "../icons/unfold.svelte";
  import AddFile from "../icons/addFile.svelte";
  import Delete from "../icons/delete.svelte";

  import type { Folder as FolderType } from "src/types";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let handleFilesSelect: any;
  export let handleFold: () => void;
  export let folderName = "";
  export let fold: boolean;
  export let folder: FolderType;
  export let handleFolderDelete: () => void = undefined;

  $: amount = folder.files.length;
</script>

<div
  class="flex sticky top-20 bg-orange-50 dark:bg-slate-800 align-middle justify-between items-center min-w-fit w-full h-16 border-b-2 border-orange-400 my-4 rounded-md z-10"
>
  <div class="ml-3 flex items-center">
    <div class="mr-4 text-orange-500">
      <Folder />
    </div>
    <div>
      {folderName} ({amount})
    </div>
  </div>
  <div class="text-orange-600 items-center flex bg-transparent">
    <IconButton onClick={handleFilesSelect}>
      <AddFile />
    </IconButton>
    <IconButton onClick={handleFolderDelete}>
      <Delete />
    </IconButton>
    {#if !fold}
      <IconButton onClick={handleFold}>
        <Unfold />
      </IconButton>
    {/if}
    {#if fold}
      <IconButton onClick={handleFold}>
        <Fold />
      </IconButton>
    {/if}
  </div>
</div>
