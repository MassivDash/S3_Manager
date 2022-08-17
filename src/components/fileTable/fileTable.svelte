<script lang="ts">
  import FolderTitle from "src/components/table/folderTitle.svelte";
  import Table from "src/components/table/table.svelte";

  import { fade } from "svelte/transition";


  export let handleFilesSelect: (value) => void;
  export let folder;
  export let bucket;
  export let handleCheckbox;
  export let checkedFiles;

  let fold = false
  function handleFold() {
    fold = !fold;
  }
</script>

<div class="bg-white">
  <FolderTitle folderName={folder.name} {handleFilesSelect} {handleFold} {fold} />
  {#if !fold}
  <div
    class="flex transition-opacity text-gray-600 ease-in duration-700"
    in:fade={{ duration: 700 }}
    out:fade={{ duration: 700 }}
  >
    <Table
      bucketName={bucket.name}
      files={folder.files}
      handleCheckbox={handleCheckbox}
      {checkedFiles}
    />
  </div>
{/if}
</div>
