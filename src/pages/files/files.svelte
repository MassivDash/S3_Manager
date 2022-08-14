<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "../../components/loader/loader.svelte";
  import Search from "src/components/search/search.svelte";
import Item from "src/components/sideMenu/Item.svelte";

  const registerFocus = useFocus();
  let response;
  let filteredList;
  let value = "";

  $: response;
  $: filteredList = response?.map((bucket) => ({
    ...bucket,
    files:
      value === ""
        ? bucket.files
        : bucket.files.filter((item) => item.name.indexOf(value) !== -1),
  }));

  interface File {
    key: string;
    name: string;
    folder: string;
    size: number;
    last_modified: number;
  }

  interface Bucket {
    name: string;
    files: File[];
    total_files: number;
  }

  console.log(filteredList, value);

  onMount(async () => {
    const res: Bucket[] = await invoke("get_buckets");
    response = res;
  });
</script>

<div use:registerFocus class="outline-none">
  {#if !filteredList}
    <div class="flex justify-center items-center w-full h-screen">
      <Loader />
    </div>
  {/if}
  {#if filteredList && filteredList[0].name}
    <div class="flex">
      <Search bind:value />
    </div>
    {#each filteredList as bucket}
      <div class="flex h-9 justify-start items-center my-4">
        <div class="w-1/4 h-1 rounded-md bg-gray-500" />
        <div class="w-1/4 text-center text-gray-500">
          bucket: {bucket.name}
          {bucket.files.length > 0 ? `(${bucket.files.length})` : ""}
        </div>
        <div class="h-1 w-2/4 rounded-md bg-gray-500" />
      </div>
      <div>
        {#each bucket.files as item}
          <div>{item.name}</div>
          <div class="text-red-900">{item.folder}</div>
        {/each}
      </div>
    {/each}
  {/if}
</div>



