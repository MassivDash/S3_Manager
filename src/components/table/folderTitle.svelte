<script lang="ts">
  import Folder from "../icons/folder.svelte";
  import AddSq from "../icons/addSquare.svelte";

  export let handleFilesSelect;
  export let folderName = "";

  
</script>

<div
  class="flex sticky top-20 bg-orange-50 align-middle justify-between items-center min-w-fit w-full h-14 border-b-2 border-orange-400 my-4 rounded-md z-10"
  >
  <div class="ml-3 flex items-center">
    <div class="mr-4 text-orange-500">
      <Folder />
    </div>
    <div class="text-gray-800">
      {folderName}
    </div>
  </div>
  <div>
  <button
    on:click={handleFilesSelect}
    type="button"
    class="mr-4 border-0 appearance-none outline-orange-500 text-orange-500 bg-none"
  >
    <AddSq />
  </button>
  </div>
</div>
