<script lang="ts">
  import Folder from "../icons/folder.svelte";
  import IconButton from "../iconButton/iconButton.svelte";
  import Fold from "../icons/fold.svelte";
  import Unfold from "../icons/unfold.svelte";
  import AddFile from "../icons/addFile.svelte";


  export let handleFilesSelect;
  export let folderName = "";
  export let handleFold;
  export let fold;

  
</script>

<div
  class="flex sticky top-20 bg-orange-50 align-middle justify-between items-center min-w-fit w-full h-16 border-b-2 border-orange-400 my-4 rounded-md z-10"
>
  <div class="ml-3 flex items-center">
    <div class="mr-4 text-orange-500">
      <Folder />
    </div>
    <div class="text-gray-800">
      {folderName}
    </div>
  </div>
  <div class="text-orange-600 items-center flex bg-transparent">
    <IconButton onClick={handleFilesSelect}>
      <AddFile />
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
