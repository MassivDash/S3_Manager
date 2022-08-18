<script lang="ts">
  import FileDrop from "svelte-tauri-filedrop";
  import { invoke } from "@tauri-apps/api";
  import { fade, fly } from "svelte/transition";
  import Select from "../select/select.svelte";
  import type { Bucket } from "src/types";

  import Loader from "../loader/loader.svelte";
  import AddFile from "src/components/icons/addFile.svelte";
  import Close from "src/components/icons/close.svelte";

  let visible = false;
  let loading = false;
  let files: string[] = [];
  let buckets: Bucket[];
  let bucketName: string;
  let folderName: string;

  $: folders =
    buckets &&
    buckets[findIndex(bucketName)]?.folders.map((folder) => folder.name);

  async function handleDrop(paths: string[]): Promise<void> {
    const res: Bucket[] = await invoke("get_files");
    buckets = res;
    files = [...new Set([...files, ...paths])];
    visible = true;
  }

  async function handleUpload(
    paths: string[],
    bucketName: string,
    folderName: string
  ): Promise<void> {
    loading = true;
    const upload = await invoke("put_files", {
      bucketName,
      folderName,
      files: paths,
    });
    if (upload) {
      files = [];
      buckets = [];
      loading = false;
      visible = false;
    }
  }

  function findIndex(bucketName: string): number {
    const index = buckets?.findIndex((bucket) => bucket.name === bucketName);
    return index;
  }
</script>

<FileDrop handleFiles={(paths) => handleDrop(paths)} let:files>
  <slot /></FileDrop
>
{#if visible}
  <div
    in:fly={{ y: 200, duration: 2000 }}
    out:fade
    class="fixed overflow-y-auto bottom-0 pl-4 pt-2 pb-4 right-0 w-72 h-4/6 z-50 bg-orange-50 dark:bg-slate-800 flex flex-col rounded-t shadow-sm justify-start items-stretch"
  >
    {#if loading}
      <div class="w-full h-full flex justify-center items-center">
        <Loader />
      </div>
    {:else}
      <div
        class="text-gray-800 dark:text-white flex justify-between items-center mb-4"
      >
        <h2>Uploading files</h2>
        <div
          class="text-gray-800 m-2 p-2 cursor-pointer dark:text-white bg-orange-50 dark:bg-slate-700 hover:bg-amber-100 hover:text-gray-50 hover:dark:bg-slate-900 hover:dark:text-orange-50"
          on:click={() => {
            visible = false;
            files = [];
          }}
        >
          <Close />
        </div>
      </div>
      <form
        on:submit|preventDefault={() =>
          handleUpload(files, bucketName, folderName)}
      >
        <Select
          bind:value={bucketName}
          label={"Choose bucket"}
          options={buckets.map((bucket) => bucket.name)}
          handleChange={() => (folderName = "")}
        />
        {#if bucketName}
          <Select
            bind:value={folderName}
            label={"Choose folder"}
            options={folders}
          />
        {/if}
        <div
          class="flex flex-col mt-8 pt-4 justify-center items-center text-gray-800 dark:text-white"
        >
          <AddFile width={48} height={48} />
          <p>{`Upload ${files.length} file${files.length > 1 ? "s" : ""} ?`}</p>
        </div>
        <div class="flex justify-center items-center w-full h-24">
          <button
            disabled={folderName === ""}
            class="m-auto p-5 w-24 text-gray-700 dark:text-white bg-orange-100 dark:bg-slate-600"
            type="submit"
          >
            Submit
          </button>
        </div>
      </form>
    {/if}
  </div>
{/if}
