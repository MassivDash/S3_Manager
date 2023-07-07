<script lang="ts">
  import Folder from "./folder.svelte";
  import FolderIcon from "../icons/folder.svelte";

  export let checkedFolders: string[] = [];
  export let handleCheckFolders: (folder: string) => void;
  export let folderList: string[] = [];
</script>

<div
  class="mr-4 mb-2 flex flex-auto gap-4 flex-wrap justify-center items-center border-2 border-orange-500 text-orange-500"
>
  <div class="ml-2"><FolderIcon /></div>
  <div class="flex flex-auto gap-4 flex-wrap">
    {#each folderList as folder (folder)}
      <Folder {checkedFolders} {handleCheckFolders} {folder} />
    {/each}
  </div>
</div>
