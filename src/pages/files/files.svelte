<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { useFocus } from "svelte-navigator";
  import Loader from "../../components/loader/loader.svelte";
  import Search from "src/components/search/search.svelte";


  interface File {
    key: string;
    name: string;
    folder: string;
    size: number;
    last_modified: number;
  }

  interface Folder {
    name: string;
    files: File[];
    total_files: number;
    total_size: number;
  }

  interface Bucket {
    name: string;
    folders: Folder[];
    total_files: number;
  }

  const registerFocus = useFocus();
  let response;
  let filteredList;
  let value = "";
  let folders = [];

  $: response;
  $: filteredList = response?.map((bucket: Bucket) => ({
    ...bucket,
    folders: 
      value === ""
        ? [...bucket.folders]
        : bucket.folders.map((folder: Folder) => ({...folder, files: folder.files.filter((item) => item.name.indexOf(value) !== -1) }) ),
  }));


  console.log(filteredList, value);

  onMount(async () => {
    const res: Bucket[] = await invoke("get_files");
    response = res;
    console.log(res);
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
      <div class="text-blue-800">{bucket.name}</div>
          {#each bucket.folders as folder}
            <div>{folder.name}</div>
            <div>
            {#each folder.files as file}
              <div class="flex h-9 justify-start items-center my-4">
                <div class="text-center text-gray-500">
                  {file.name}
                </div>
              </div>
            {/each}
          </div>
          {/each}
    {/each}
  {/if}
</div>
