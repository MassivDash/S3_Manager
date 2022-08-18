<script lang="ts">
  import FolderTitle from "src/components/table/folderTitle.svelte";
  import Table from "src/components/table/table.svelte";

  import { fade } from "svelte/transition";
  import type { Bucket, Folder, CheckedFile } from "src/types";

  export let handleFilesSelect: (
    bucketName: string,
    folderName: string
  ) => void;
  export let folder: Folder;
  export let bucket: Bucket;
  export let handleCheckbox: (key: string, bucketName: string) => void;
  export let checkedFiles: CheckedFile[];

  let fold = false;
  function handleFold(): void {
    fold = !fold;
  }
</script>

<div class="bg-white">
  <FolderTitle
    folderName={folder.name}
    {handleFilesSelect}
    {handleFold}
    {fold}
    {folder}
  />
  {#if !fold}
    <div
      class="flex transition-opacity text-gray-600 ease-in duration-700"
      in:fade={{ duration: 700 }}
      out:fade={{ duration: 700 }}
    >
      <Table
        bucketName={bucket.name}
        files={folder.files}
        {handleCheckbox}
        {checkedFiles}
      />
    </div>
  {/if}
</div>
