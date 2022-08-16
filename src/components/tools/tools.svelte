<script lang="ts">
  import IconButton from "src/components/iconButton/iconButton.svelte";
  import Search from "src/components/search/search.svelte";
  import Download from "src/components/icons/download.svelte";
  import Delete from "src/components/icons/delete.svelte";

  import {invoke} from "@tauri-apps/api";
  import { fade } from "svelte/transition";

  import { confirm, open } from "@tauri-apps/api/dialog";
  export let checkedFiles;
  export let value;

  async function handleDownload(checkedFiles) {
    const dirPath = await open({
      directory: true,
      title: "Select a directory",
    });
    if (dirPath) {
      console.log(checkedFiles)
      await invoke("save_files", {
        keys: checkedFiles,
        dir: dirPath
      });
    }
  }

  async function handleDelete(checkedFiles) {
    const confirmed = await confirm(
      "This action cannot be reverted. Are you sure you want to delete?",
      { title: "Delete files ?", type: "warning" }
    );
    if (confirmed) {
      console.log("deleting", checkedFiles);
    }
  }
</script>

<div
  class="fixed w-11/12 justify-between flex items-center h-20 top-0 bg-gray-100 z-30"
>
  <Search
    bind:value
    hidelabel="true"
    class="placeholder-gray-700 bg-orange-50 appearance-none outline-none border-2 border-transparent border-spacing-1  focus:border-orange-600 rounded text-gray-900 p-2"
  />

  {#if checkedFiles && checkedFiles.length > 0}
    <div
      id="tool"
      class="flex transition-opacity ease-in duration-700"
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

<style>
  #tools {
    color: red;
  }
</style>
